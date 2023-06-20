#![allow(dead_code, unused_variables, unused_imports)]

use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro2::{TokenTree, Group, Delimiter};
use syn::{braced, parse::{Parse, ParseStream, ParseBuffer}, Ident, token, Token, Expr};

pub trait ToHTML {
    fn render(&self) -> String;
}

impl ToHTML for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl ToHTML for String {
    fn render(&self) -> String {
        self.clone()
    }
}

impl ToHTML for Vec<Box<dyn ToHTML>> {
    fn render(&self) -> String {
        let mut rv = "".to_string();
        for elem in self.into_iter() {
            rv.push_str(elem.render().as_str())
        }
        rv
    }
}

pub trait BuildElement {
    fn add_key_val(self, attribute: &str, value: &str) -> Self;
    fn add_value(self, value: &str) -> Self;
    fn add_content<C: ToHTML + 'static>(self, child: C) -> Self;
}

pub struct Element {
    pub tag: String,
    pub attr: String,
    pub content: String,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            attr: "".to_string(),
            content: "".to_string(),
        }
    }
}

impl Parse for Element {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self::new("a"))
    }
}

impl ToHTML for Element {
    fn render(&self) -> String {
        match self.content.len() > 0 {
            true => format!("<{}{}>{}</{}>", self.tag, self.attr, self.content, self.tag),
            false => format!("<{}{}/>", self.tag, self.attr),
        }
    }
}

impl BuildElement for Element {
    fn add_value(mut self, value: &str) -> Self {
        self.attr.push_str(" ");
        self.attr.push_str(value);
        self
    }

    fn add_key_val(mut self, key: &str, value: &str) -> Self {
        self.attr.push_str(format!(" {}=\"{}\"", key.to_string(), value.to_string()).as_str());
        self
    }

    fn add_content<C: ToHTML + 'static>(mut self, child: C) -> Self {
        self.content.push_str(child.render().as_str());
        self
    }
}

#[derive(Debug)]
pub enum ContentStream {
    Text(Expr),
    Node(Box<ElementStream>),
}

impl ContentStream {
    pub fn new_text(s: Expr) -> Self {
        Self::Text(s)
    }

    pub fn new_node(s: ElementStream) -> Self {
        Self::Node(Box::new(s))
    }
}

impl Parse for ContentStream {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(token::Brace) {
            Ok(Self::new_node(input.parse()?))
        } else if input.peek(Token!($)) && input.peek2(token::Brace) {
            let _: Token![$] = input.parse()?;

            let braced_stream: ParseBuffer;
            let _ = braced!(braced_stream in input);

            Ok(Self::new_text(braced_stream.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "neither element or text"))
        }
    }
}

impl ToTokens for ContentStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Text(text) => {
                tokens.append_all(quote! { (#text).to_string() });
            },
            Self::Node(node) => {
                tokens.append_all(node.to_token_stream());
            },
        }
    }
}

#[derive(Debug)]
pub enum AttrStream {
    KeyVal(Ident, Expr),
    Val(Ident)
}

impl Parse for AttrStream {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(ident) = input.parse::<Ident>() {
            if input.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                let lit: Expr = input.parse()?;
                Ok(Self::KeyVal(ident, lit))
            } else {
                Ok(Self::Val(ident))
            }
        } else {
            Err(syn::Error::new(input.span(), "invalid attr"))
        }
    }
}

impl ToTokens for AttrStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::KeyVal(k, v) => {
                let k = k.to_string();
                tokens.append_all(quote! { format!(" {}=\"{}\"", #k, #v) });
            },
            Self::Val(v) => {
                let v = v.to_string();
                tokens.append_all(quote! { format!(" {}", #v) })
            },
        }
    }
}

#[derive(Debug)]
pub struct ElementStream {
    pub tag: Ident,
    pub attrs: Vec<AttrStream>,
    pub content: Vec<Box<ContentStream>>,
}

impl ElementStream {
    pub fn new(tag: Ident) -> Self {
        Self {
            tag,
            attrs: vec![],
            content: vec![]
        }
    }
}

impl Parse for ElementStream  {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        if let Ok(ident) = stream.parse::<Ident>() {
            let braced_stream: ParseBuffer;
            let _ = braced!(braced_stream in stream);

            let mut element = ElementStream::new(ident);

            while !braced_stream.is_empty() {
                // is content, attr, or error
                if braced_stream.peek(Ident) && braced_stream.peek2(token::Brace) || braced_stream.peek(Token![$]) {
                    element.content.push(braced_stream.parse()?);
                } else if braced_stream.peek(Ident) && braced_stream.peek2(Token![:]) {
                    element.attrs.push(braced_stream.parse()?);
                } else if braced_stream.peek(Token![,]) {
                    let _: Token![,] = braced_stream.parse()?;
                } else {
                    return Err(braced_stream.error("neither attr nor content"))
                }
            };

            Ok(element)
        } else {
            Err(stream.error("invalid element"))
        }
    }
}

impl ToTokens for ElementStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag = self.tag.to_string();
        let attrs = &self.attrs;
        let content = &self.content;
        tokens.append_all(quote! {
            format!("<{}{}>{}<{}/>",
                #tag,

                {
                    let list: Vec<String> = vec![#(#attrs),*];
                    list.join("")
                },

                {
                    let list: Vec<String> = vec![#(#content),*];
                    list.join("")
                },

                #tag)
        })
    }
}

