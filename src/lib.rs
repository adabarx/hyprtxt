#![allow(dead_code)]
use proc_macro::TokenStream;
use quote::ToTokens;

mod element;
use element::ContentStream;


#[proc_macro]
pub fn hyprtxt(input: TokenStream) -> TokenStream {
    match syn::parse::<ContentStream>(input) {
        Ok(cs) => cs.to_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}


