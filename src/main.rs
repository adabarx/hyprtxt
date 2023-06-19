use html_gen::elements::{HTML, Head, Body, P, AddAttribute, AddContent, ToHTML, Meta, Title};

#[derive(Clone)]
struct Doctype;

impl ToHTML for Doctype {
    fn render(&self) -> String {
        "<!DOCTYPE html>".to_string()
    }
}

fn main() {
    let page: Vec<Box<dyn ToHTML>> = vec![Box::new(Doctype)];

    let page = page.add_content(HTML::new()
        .add_attribute("lang", "en")
        .add_content(Head::new()
            .add_content(Meta::new().add_attribute("charset", "UTF-8"))
            .add_content(Meta::new()
                .add_attribute("name", "viewport")
                .add_attribute("content", "width=device-width"))
            .add_content(Title::new().add_content("Will this work???")))
        .add_content(Body::new()
            .add_content(P::new().add_content("This is some content"))));

    if let Ok(()) = write_to_file(page.render()) {
        println!("{}", page.render())
    } else {
        println!("booooooo...")
    };
}

fn write_to_file(input: String) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("foo.html")?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
