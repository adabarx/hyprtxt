use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{ Parse, ParseStream, ParseBuffer }, 
    braced, Ident, token, Token, Expr
};


#[proc_macro]
pub fn hyprtxt(input: TokenStream) -> TokenStream {
    match syn::parse::<ContentStream>(input) {
        Ok(cs) => cs.to_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}


#[derive(Debug)]
enum ContentStream {
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
enum AttrStream {
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
            Err(input.error("invalid attr"))
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
struct ElementStream {
    tag: Ident,
    attrs: Vec<AttrStream>,
    content: Vec<Box<ContentStream>>,
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
        if self.content.len() > 0 {
            tokens.append_all(quote! {
                format!("<{}{}>{}</{}>",
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
        } else {
            tokens.append_all(quote! {
                format!("<{}{}/>",
                    #tag,

                    {
                        let list: Vec<String> = vec![#(#attrs),*];
                        list.join("")
                    })
            })
        }
    }
}

