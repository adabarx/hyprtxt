#![allow(dead_code)]
use proc_macro::TokenStream;
// use quote::quote;
use syn::parse_macro_input;

mod element;
use element::ContentStream;


#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    println!("html!");
    let ast = parse_macro_input!(input as ContentStream);
    dbg!(ast);
    TokenStream::new()
}


