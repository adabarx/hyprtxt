#![allow(dead_code)]
use std::collections::HashMap;

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

pub trait AddAttribute {
    fn add_attribute(self, attribute: &str, value: &str) -> Self;
}

pub trait AddContent{
    fn add_content<C: ToHTML + 'static>(self, child: C) -> Self;
}

impl AddContent for Vec<Box<dyn ToHTML>> {
    fn add_content<C: ToHTML + 'static>(mut self, child: C) -> Self {
        self.push(Box::new(child));
        self
    }
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

                match (attributes.len() > 0, content.len() > 0) {
                    (true, true) =>
                        format!("<{} {}>{}</{}>", self.tag, attributes, content, self.tag),
                    (true, false) =>
                        format!("<{} {}/>", self.tag, attributes),
                    (false, true) =>
                        format!("<{}>{}</{}>", self.tag, content, self.tag),
                    (false, false) =>
                        format!("<{}/>", self.tag),
                }
            }
        }

        impl AddAttribute for $element {
            fn add_attribute(mut self, attribute: &str, value: &str) -> Self {
                let attribute = attribute.to_string();
                let value = value.to_string();
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
impl_render_attribute_content! (HTML, Head, Body, Link, Meta, Style, Title);

// content sectioning
impl_render_attribute_content!(Address, Article, Aside, Footer,
    Header, HGroup, Main, Nav, Section, H1, H2, H3, H4, H5, H6);

// text content
impl_render_attribute_content!(BlockQuote, DD, Div, DL,
    DT, Figure, FigCaption, HR, LI, Menu, P, Pre, UL);

// inline text semantics
impl_render_attribute_content!(A, Abbr, B, BDI, BDO, BR, Cite, Code, Data, Del, DFN, Em,
    I, Ins, KBD, Mark, Q, S, Samp, Small, Span, Strong, Sub, Sup, Time, U, Var, WBR);

// multimedia
impl_render_attribute_content!(Audio, Area, Img, Map, Track, Video);

// embedded content
impl_render_attribute_content!(Embed, IFrame, Object, Picture, Source);

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



