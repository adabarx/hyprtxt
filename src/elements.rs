#![allow(dead_code)]
use std::collections::HashMap;

pub trait ToHTML {
    fn render(&self) -> String;
}

impl ToHTML for String {
    fn render(&self) -> String {
        self.clone()
    }
}

pub trait AddAttribute {
    fn add_attribute(self, attribute: String, value: String) -> Self;
}

pub trait AddContent{
    fn add_content<C: ToHTML + 'static>(self, child: C) -> Self;
}

macro_rules! impl_render_attribute {
    ($($element:ident),+) => {
        $(
        pub struct $element {
            pub tag: String,
            pub attributes: HashMap<String, String>,
        }

        impl $element {
            pub fn new() -> Self {
                Self {
                    tag: stringify!($element).to_lowercase(),
                    attributes: HashMap::new(),
                }
            }
        }
        
        impl ToHTML for $element {
            fn render(&self) -> String {
                let attributes = self.attributes.iter()
                    .map(|(attr, val)|
                        if attr == val { attr.clone() } 
                        else { format!("{}=\"{}\"", attr, val) })
                    .collect::<Vec<String>>()
                    .join(" ");

                format!("<{} {} />", self.tag, attributes)
            }
        }

        impl AddAttribute for $element {
            fn add_attribute(mut self, attribute: String, value: String) -> Self {
                if let Some(attr) = self.attributes.get_mut(&attribute) {
                    attr.push_str(format!(" {}", value).as_str());
                } else {
                    self.attributes.insert(attribute, value);
                }
                self
            }
        }
        )+
    };
}

macro_rules! impl_render_attribute_content {
    ($($element:ident),+) => {
        $(
        pub struct $element {
            pub tag: String,
            pub attributes: HashMap<String, String>,
            pub content: Vec<Box<dyn ToHTML>>,
        }

        impl $element {
            pub fn new() -> Self {
                Self {
                    tag: stringify!($element).to_lowercase(),
                    attributes: HashMap::new(),
                    content: Vec::new(),
                }
            }
        }
        
        impl ToHTML for $element {
            fn render(&self) -> String {
                let attributes = self.attributes.iter()
                    .map(|(attr, val)|
                        if attr == val { attr.clone() } 
                        else { format!("{}=\"{}\"", attr, val) })
                    .collect::<Vec<String>>()
                    .join(" ");

                let content = self.content.iter()
                    .map(|child| child.render())
                    .collect::<Vec<String>>()
                    .join("");

                format!("<{} {}>{}</{}>", self.tag, attributes, content, self.tag)
            }
        }

        impl AddAttribute for $element {
            fn add_attribute(mut self, attribute: String, value: String) -> Self {
                if let Some(attr) = self.attributes.get_mut(&attribute) {
                    attr.push_str(format!(" {}", value).as_str());
                } else {
                    self.attributes.insert(attribute, value);
                }
                self
            }
        }

        impl AddContent for $element {
            fn add_content<C: ToHTML + 'static>(mut self, child: C) -> Self {
                self.content.push(Box::new(child));
                self
            }
        }
        )+
    };
}

// basic HTML tags
impl_render_attribute_content!
    (HTML, Head, Body, Style, Title);
impl_render_attribute!
    (Link, Meta);

// content sectioning
impl_render_attribute_content!(Address, Article, Aside,
    Footer, Header, HGroup, Main, Nav, Section);
impl_render_attribute_content!(H1, H2, H3, H4, H5, H6);

// text content
impl_render_attribute_content!(BlockQuote, DD, Div, DL, DT,
    Figure, FigCaption, HR, LI, Menu, P, Pre, UL);

// inline text semantics
impl_render_attribute_content!(A, Abbr, B, BDI, BDO, BR, Cite, Code, Data, Del, DFN, Em, I, Ins,
    KBD, Mark, Q, S, Samp, Small, Span, Strong, Sub, Sup, Time, U, Var, WBR);

// multimedia
impl_render_attribute_content!(Audio, Map, Video);
impl_render_attribute!(Area, Img, Track);

// embedded content
impl_render_attribute_content!(IFrame, Object, Picture);
impl_render_attribute!(Embed, Source);

// scripting
impl_render_attribute_content!(Canvas, NoScript, Script);

// table contents
impl_render_attribute_content!(Caption, Col, ColGroup, Table, TBody, TD, TFoot, TH, THead, TR);

// forms
impl_render_attribute_content!(Button, DataList, FieldSet, Form, Input, Label, Legend, Meter,
    OptGroup, Option, Output, Progress, Select, TextArea);

// interactive elements
impl_render_attribute_content!(Details, Dialog, Summary);

// TODO: WebComponents

