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

pub trait ElementTag {
    fn as_tag(&self) -> String;
}

pub enum ElementType {
    Formatting(Formatting),
}

impl ElementTag for ElementType {
    fn as_tag(&self) -> String {
        match self {
            Self::Formatting(f) => f.as_tag(),
        }
    }
}

pub enum Formatting {
    Abbr,
    Address,
    B,
    Bdi,
    BlockQuote,
}

impl ElementTag for Formatting {
    fn as_tag(&self) -> String {
        use Formatting::*;
        match self {
            Abbr => "abbr".to_string(),
            Address => "address".to_string(),
            B => "b".to_string(),
            Bdi => "bdi".to_string(),
            BlockQuote => "blockquote".to_string(),
        }
    }
}

pub struct FormattingElement {
    pub element: Formatting,
    pub attributes: HashMap<String, String>,
    pub content: Vec<Box<dyn ToHTML>>,
}

impl ToHTML for FormattingElement {
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

        let tag = self.element.as_tag();
        format!("<{} {}>{}</{}>", tag, attributes, content, tag)
    }
}

impl AddAttribute for FormattingElement {
    fn add_attribute(mut self, attribute: String, value: String) -> Self {
        if let Some(attr) = self.attributes.get_mut(&attribute) {
            attr.push_str(format!(" {}", value).as_str());
        } else {
            self.attributes.insert(attribute, value);
        }
        self
    }
}

impl AddContent for FormattingElement {
    fn add_content<C: ToHTML + 'static>(mut self, child: C) -> Self {
        self.content.push(Box::new(child));
        self
    }
}
