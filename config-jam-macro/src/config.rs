use proc_macro::TokenStream;
use proc_macro2::Span;

pub fn config(item: TokenStream) -> syn::Result<TokenStream> {
    let config_struct: Config = syn::parse(item)?;

    let mut ts = proc_macro2::TokenStream::new();
    use std::iter::FromIterator;
    use syn::spanned::Spanned;
    let mut field_derive = config_struct
        .attrs
        .iter()
        .find(|&attr| attr.path.is_ident("field_derive"))
        .cloned();
    if let Some(ref mut field_derive) = field_derive {
        field_derive.path = syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(
                vec![PathSegment {
                    ident: Ident::new("derive", field_derive.span()),
                    arguments: syn::PathArguments::None,
                }]
                .into_iter(),
            ),
        };
    }

    let mut get_block = quote! {};

    for f in config_struct.fields.named.iter() {
        let field_name = &f.ident;

        if let Some(ref default_value) = f.default_value {
            // An Optional Field

            let field_struct_ident = field_struct_ident(&config_struct.ident, field_name);
            let default_static_ident = default_static_ident(&config_struct.ident, field_name);

            let ty = &f.ty;
            let default_expr = &default_value.value;
            let field_struct = quote! {
                #field_derive
                pub struct #field_struct_ident(#ty);

                impl From<#ty> for #field_struct_ident {
                    fn from(val: #ty) -> Self {
                        Self(val)
                    }
                }
                impl Into<#ty> for #field_struct_ident {
                    fn into(self) -> #ty {
                        self.0
                    }
                }

                impl Default for #field_struct_ident {
                    fn default() -> Self {
                        Self(#default_expr)
                    }
                }
                impl std::ops::Deref for #field_struct_ident {
                    type Target = #ty;

                    fn deref(&self) -> &#ty {
                        &self.0
                    }
                }
                impl std::ops::DerefMut for #field_struct_ident {
                    fn deref_mut(&mut self) -> &mut #ty {
                        &mut self.0
                    }
                }
                config_jam::lazy_static! {
                    static ref #default_static_ident : #field_struct_ident = #field_struct_ident::default();
                }

            };

            ts.extend(field_struct);

            get_block = quote! {
                #get_block
                #field_name: self.#field_name.as_ref().unwrap_or(&#default_static_ident),
            };
        } else {
            // A Required Field
            get_block = quote! {
                #get_block
                #field_name: &self.#field_name,
            };
        }
    }

    let item_struct = config_struct.to_struct();
    let item_struct_view = config_struct.to_struct_view();

    let item_struct_ident = &item_struct.ident;
    let item_struct_view_ident = &item_struct_view.ident;

    ts.extend(quote! {
        #item_struct


        impl #item_struct_ident {
            pub fn get<'a>(&'a self) -> #item_struct_view_ident<'a> {
                #item_struct_view_ident {
                    #get_block
                }
            }
        }

        #[derive(Clone, Debug)]
        #item_struct_view

    });

    // println!("res {}", ts.to_string());

    Ok(ts.into())
}

use syn::{
    parse::Parse, parse::ParseStream, punctuated::Punctuated, token::Brace, token::Colon,
    Attribute, Expr, Generics, Ident, Lifetime, PathSegment, Result, Type, TypeReference,
    Visibility,
};

mod kw {
    syn::custom_keyword!(config);
}

pub struct Config {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub config_token: kw::config,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: ConfigFields,
}

impl Config {
    fn to_struct(&self) -> syn::ItemStruct {
        syn::ItemStruct {
            attrs: self
                .attrs
                .iter()
                .filter(|attr| !attr.path.is_ident("field_derive"))
                .cloned()
                .collect(),
            vis: self.vis.clone(),
            struct_token: Token![struct](self.config_token.span.clone()),
            ident: self.ident.clone(),
            generics: self.generics.clone(),
            fields: syn::Fields::Named(self.fields.to_fields_named(&self.ident)),
            semi_token: None,
        }
    }

    fn to_struct_view(&self) -> syn::ItemStruct {
        let lt = Lifetime {
            apostrophe: Span::call_site(),
            ident: Ident::new("a", Span::call_site()),
        };
        let mut generics = self.generics.clone();
        generics.params.insert(
            0,
            syn::GenericParam::Lifetime(syn::LifetimeDef::new(lt.clone())),
        );

        syn::ItemStruct {
            attrs: Vec::new(),
            vis: self.vis.clone(),
            struct_token: Token![struct](self.config_token.span.clone()),
            ident: Ident::new(&format!("{}{}", &self.ident, "View"), self.ident.span()),
            generics: generics,
            fields: syn::Fields::Named(self.fields.to_lifetime_fields_named(&lt)),
            semi_token: None,
        }
    }
}

pub struct ConfigFields {
    pub brace_token: Brace,
    pub named: Punctuated<ConfigField, Token![,]>,
}

impl ConfigFields {
    pub fn to_fields_named(&self, struct_ident: &Ident) -> syn::FieldsNamed {
        syn::FieldsNamed {
            brace_token: self.brace_token.clone(),
            named: self
                .named
                .iter()
                .map(|f| f.to_field(&struct_ident))
                .collect(),
        }
    }

    pub fn to_lifetime_fields_named(&self, lifetime: &Lifetime) -> syn::FieldsNamed {
        syn::FieldsNamed {
            brace_token: self.brace_token.clone(),
            named: self
                .named
                .iter()
                .map(|f| f.to_lifetimed_field(lifetime))
                .collect(),
        }
    }
}

pub struct ConfigField {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident, // This is not optional compared to syn Fields.
    pub colon_token: Option<Colon>,
    pub ty: Type,
    pub default_value: Option<ConfigDefaultValue>,
}

impl ConfigField {
    pub fn to_field(&self, struct_ident: &Ident) -> syn::Field {
        let ty = self.ty.clone();
        let ty = if self.default_value.is_some() {
            let field_struct_name = field_struct_ident(&struct_ident, &self.ident);
            Type::Path(syn::parse2(quote! { Option<#field_struct_name> }).expect("Invalid path!"))
        } else {
            ty
        };
        syn::Field {
            attrs: self.attrs.clone(),
            vis: self.vis.clone(),
            ident: Some(self.ident.clone()),
            colon_token: self.colon_token.clone(),
            ty: ty,
        }
    }

    pub fn to_lifetimed_field(&self, lifetime: &Lifetime) -> syn::Field {
        syn::Field {
            attrs: self.attrs.clone(),
            vis: self.vis.clone(),
            ident: Some(self.ident.clone()),
            colon_token: self.colon_token.clone(),
            ty: Type::Reference(TypeReference {
                and_token: Token![&](Span::call_site()),
                lifetime: Some(lifetime.clone()),
                mutability: None,
                elem: Box::new(self.ty.clone()),
            }),
        }
    }
}

pub struct ConfigDefaultValue {
    pub eq: Token![=],
    pub value: Expr,
}

impl Parse for Config {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Config {
            attrs: Attribute::parse_outer(input)?,
            vis: input.parse()?,
            config_token: input.parse()?,
            ident: input.parse()?,
            generics: input.parse()?,
            fields: input.parse()?,
        })
    }
}

impl Parse for ConfigFields {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        Ok(ConfigFields {
            brace_token: braced!(content in input),
            named: content.parse_terminated(ConfigField::parse)?,
        })
    }
}

impl Parse for ConfigField {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = Attribute::parse_outer(input)?;
        let vis = input.parse()?;
        let ident = input.parse()?;
        let colon_token = input.parse()?;
        let ty = input.parse()?;
        let default_value = if input.peek(Token![=]) {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(ConfigField {
            attrs,
            vis,
            ident,
            colon_token,
            ty,
            default_value,
        })
    }
}

impl Parse for ConfigDefaultValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ConfigDefaultValue {
            eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

fn field_struct_ident(struct_ident: &Ident, field_name: &Ident) -> Ident {
    use heck::CamelCase;
    Ident::new(
        &format!(
            "{}{}",
            &struct_ident,
            &field_name.to_string().to_camel_case()
        ),
        field_name.span(),
    )
}

fn default_static_ident(struct_ident: &Ident, field_name: &Ident) -> Ident {
    use heck::ShoutySnakeCase;
    Ident::new(
        &format!(
            "DEFAULT_{}_{}",
            &struct_ident.to_string().to_shouty_snake_case(),
            &field_name.to_string().to_shouty_snake_case()
        ),
        field_name.span(),
    )
}
