#![allow(unused)]
use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{ Parse, ParseStream, ParseBuffer, Parser },
    braced, Ident, token, Token, Expr,
    ext::IdentExt,
};


#[proc_macro]
pub fn hyprtxt(input: TokenStream) -> TokenStream {
    match syn::parse::<ElementStream>(input) {
        Ok(cs) => cs.to_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}


#[derive(Debug)]
struct ContentStream(Expr);


impl Parse for ContentStream {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token!($)) && input.peek2(Token!(:)) {
            let _: Token![$] = input.parse()?;
            let _: Token![:] = input.parse()?;

            Ok(Self(input.parse()?))
        } else {
            Err(input.error("invalid content"))
        }
    }
}

impl ToTokens for ContentStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let text = &self.0;
        tokens.append_all(quote! { (#text).to_string() });
    }
}

#[derive(Debug)]
struct AttrStream(Ident, Expr);

impl Parse for AttrStream {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        if stream.peek(Token!(.)) {
            let _: Token!(.) = stream.parse()?;
            let _: Token!(=) = stream.parse()?;
            return Ok(Self(Ident::parse_any.parse_str("class")?, stream.parse::<Expr>()?))
        }

        if stream.peek(Token!(#)) {
            let _: Token!(#) = stream.parse()?;
            let _: Token!(=) = stream.parse()?;
            return Ok(Self(Ident::parse_any.parse_str("id")?, stream.parse::<Expr>()?))
        }

        let ident = stream.call(Ident::parse_any)?;
        let _: Token!(=) = stream.parse()?;
        let value: Expr = stream.parse()?;

        Ok(Self(ident, value))
    }
}

impl ToTokens for AttrStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let key = self.0.to_string();
        let value = &self.1;
        tokens.append_all(quote! { format!(" {}=\"{}\"", #key, #value) });
    }
}

struct ElementStream {
    tag: Ident,
    void: bool,
    attrs: Vec<AttrStream>,
    content: Vec<Box<dyn ToTokens>>,
}

impl ElementStream {
    pub fn new(tag: Ident, void: bool) -> Self {
        Self {
            tag,
            void,
            attrs: vec![],
            content: vec![]
        }
    }
}

enum ElemTypeResult {
    Void,
    Content,
    Default,
}

impl ElemTypeResult {
    pub fn new(s: &ParseBuffer) -> syn::Result<Self> {
        if s.peek(token::Brace) {
            return Ok(Self::Default)
        };

        if s.peek(Token!(*)) {
            return Ok(Self::Void)
        };

        if s.peek(Token!(:)) {
            return Ok(Self::Content)
        }

        Err(s.error("Invalid Element"))
    }
}

enum PeekInsideElemResult {
    Element,
    Content,
    Attribute,
    Class,
    Id,
}

impl PeekInsideElemResult {
    pub fn new(s: &ParseBuffer) -> syn::Result<Self> {
        // p {} -> <p></p>
        if s.peek(Ident::peek_any) && s.peek2(token::Brace) {
            return Ok(Self::Element)
        };
        // p: "x" -> <p>x</p>
        if s.peek(Ident::peek_any) && s.peek2(Token!(:)) {
            return Ok(Self::Element)
        };
        // { $: "x" } -> <>x</>
        if s.peek(Token!($)) && s.peek2(Token!(:)) {
            return Ok(Self::Content)
        };
        // p { class="stuff" } -> <p class="stuff"></p>
        if s.peek(Ident::peek_any) && s.peek2(Token!(=)) {
            return Ok(Self::Attribute)
        };
        // p { .="stuff" } -> <p class="stuff"></p>
        if s.peek(Token!(.)) && s.peek2(Token!(=)) {
            return Ok(Self::Class)
        };
        // p { #="stuff" } -> <p id="stuff"></p>
        if s.peek(Token!(#)) && s.peek2(Token!(=)) {
            return Ok(Self::Id);
        };
        if s.peek(Ident::peek_any) && s.peek2(Token!(*)) && s.peek3(token::Brace) {
            return Ok(Self::Element)
        };
        Err(s.error("Invalid Syntax"))
    }
}


impl Parse for ElementStream  {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let ident = stream.call(Ident::parse_any)?;

        let mut element = match ElemTypeResult::new(stream)? {
            ElemTypeResult::Default => ElementStream::new(ident, false),
            ElemTypeResult::Void => {
                let _: Token!(*) = stream.parse()?;
                ElementStream::new(ident, true)
            },
            ElemTypeResult::Content => {
                let _: Token!(:) = stream.parse()?;
                let mut element = ElementStream::new(ident, false);
                element.content.push(Box::new(stream.parse::<Expr>()?));
                return Ok(element)
            },

        };
        
        let b_stream: ParseBuffer;
        let _ = braced!(b_stream in stream);

        while !b_stream.is_empty() {
            use PeekInsideElemResult::*;
            match PeekInsideElemResult::new(&b_stream)? {
                Content =>
                    element.content.push(Box::new(b_stream.parse::<ContentStream>()?)),
                Element =>
                    element.content.push(Box::new(b_stream.parse::<ElementStream>()?)),
                Id | Class | Attribute => element.attrs.push(b_stream.parse()?),
            }
        };

        Ok(element)
    }
}

impl ToTokens for ElementStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag = self.tag.to_string();
        let attrs = &self.attrs;
        let content = &self.content;
        if self.void {
            tokens.append_all(quote! {
                format!("<{}{}>",
                    #tag,

                    {
                        let list: Vec<String> = vec![#(#attrs),*];
                        list.join("")
                    })
            })
        } else if self.content.len() == 0 {
            tokens.append_all(quote! {
                format!("<{}{}/>",
                    #tag,

                    {
                        let list: Vec<String> = vec![#(#attrs),*];
                        list.join("")
                    })
            })
        } else {
            tokens.append_all(quote! {
                format!("<{}{}>{}</{}>",
                    #tag,

                    {
                        let list: Vec<String> = vec![#(#attrs.into()),*];
                        list.join("")
                    },

                    {
                        let list: Vec<String> = vec![#(#content.into()),*];
                        list.join("")
                    },

                    #tag)
            })
        }
    }
}

