use proc_macro::TokenStream;
use syn::DeriveInput;

pub fn override_with(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse(input)?;

    let mut override_with_block = quote! {
        use config_jam::OverrideWith;

    };
    if let syn::Data::Struct(ref struct_def) = input.data {
        let struct_ident = &input.ident;
        for f in struct_def.fields.iter() {
            let field_ident = &f.ident;
            override_with_block = quote! {
                #override_with_block
                self.#field_ident.override_with(other.#field_ident);
            }
        }

        Ok(TokenStream::from(quote! {
            impl config_jam::OverrideWith for #struct_ident {
                fn override_with(&mut self, other: Self) {
                    #override_with_block
                }
            }
        }))
    } else {
        use syn::spanned::Spanned;

        Err(syn::Error::new(
            input.span(),
            "Derive OverrideWith only works on structs",
        ))
    }
}
