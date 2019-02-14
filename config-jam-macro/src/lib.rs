// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod config;
mod override_with;

use proc_macro::TokenStream;

#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    config::config(item).unwrap_or_else(|e| TokenStream::from(e.to_compile_error()))
}

#[proc_macro_derive(OverrideWith)]
pub fn override_with(input: TokenStream) -> TokenStream {
    override_with::override_with(input).unwrap_or_else(|e| TokenStream::from(e.to_compile_error()))
}
