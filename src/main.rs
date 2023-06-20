#![allow(dead_code)]
use html_gen::html;

fn main() {
    html!(html {
        head {
            meta { test: "attribute" },
            meta { test: "attribute2", foo: "attribute3" }
        },
        body {
            p {
                $ { "this is the content" },
            },
        },
    })
}

fn write_to_file(input: String) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("foo.html")?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
