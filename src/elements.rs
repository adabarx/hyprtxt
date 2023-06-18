use std::collections::HashMap;

pub trait ToHTML {
    fn render(&self) -> String;
}

impl ToHTML for String {
    fn render(&self) -> String {
        self.clone()
    }
}

pub trait BuildElement {
    fn add_attribute(self, attribute: String, value: String) -> Self;
    fn add_tag(self, tag: String) -> Self;
    fn add_child<C: ToHTML + 'static>(self, child: C) -> Self;
}

macro_rules! impl_render {
    ($($elem:ident),*) => {
        $(
            #[derive(Default)]
            pub struct $elem {
                children: Vec<Box<dyn ToHTML>>,
                attributes: HashMap<String, String>,
                tags: Vec<String>
            }

            impl ToHTML for $elem {
                fn render(&self) -> String {
                    let tags = self.tags.join(" ");

                    let attributes = self.attributes.iter()
                        .map(|(attr, val)| format!("{}=\"{}\"", attr, val))
                        .collect::<Vec<String>>()
                        .join(" ");

                    let children = self.children.iter()
                        .map(|child| child.render())
                        .collect::<Vec<String>>()
                        .join("");

                    format!("<div {} {}>{}</div>", attributes, tags, children)
                }
            }

            impl BuildElement for $elem {
                fn add_attribute(mut self, attribute: String, value: String) -> Self {
                    if let Some(attr) = self.attributes.get_mut(&attribute) {
                        attr.push_str(format!(" {}", value).as_str());
                    } else {
                        self.attributes.insert(attribute, value);
                    }
                    self
                }

                fn add_tag(mut self, tag: String) -> Self {
                    self.tags.push(tag);
                    self
                }

                fn add_child<C: ToHTML + 'static>(mut self, child: C) -> Self {
                    self.children.push(Box::new(child));
                    self
                }
            }
        )*
    };
}

impl_render!(Div, Paragraph,);

// impl ToHTML for Div {
//     fn render(&self) -> String {
//         let tags = self.tags.join(" ");
//
//         let attributes = self.attributes.iter()
//             .map(|(attr, val)| format!("{}=\"{}\"", attr, val))
//             .collect::<Vec<String>>()
//             .join(" ");
//
//         let children = self.children.iter()
//             .map(|&child| child.render())
//             .collect::<Vec<String>>()
//             .join("");
//
//         format!("<div {} {}>{}</div>", attributes, tags, children)
//     }
// }
//
// impl BuildElement for Div {
//     fn add_attribute(mut self, attribute: String, value: String) -> Self {
//         if let Some(attr) = self.attributes.get_mut(&attribute) {
//             attr.push_str(format!(" {}", value).as_str());
//         } else {
//             self.attributes.insert(attribute, value);
//         }
//         self
//     }
//
//     fn add_tag(mut self, tag: String) -> Self {
//         self.tags.push(tag);
//         self
//     }
//
//     fn add_child<C: ToHTML>(mut self, child: C) -> Self {
//         self.children.push(Box::new(child));
//         self
//     }
// }
