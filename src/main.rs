#![allow(dead_code)]
use hyprtxt::hyprtxt;

fn main() {
    let x = hyprtxt!(
        html {
            lang: "en"
            head {
                meta { charset: "UTF-8" }
                meta { name: "viewport", content: "width=device-width" }
                title { $: "Will this work???" }
            }
            body {
                p {
                    $: "this is the content"
                    $: "this is moar"
                }
            }
        }
    );
    dbg!(x);
}

fn write_to_file(input: String) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("foo.html")?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
