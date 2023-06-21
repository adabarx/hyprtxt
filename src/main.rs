#![allow(dead_code)]
use hyprtxt::hyprtxt;

fn main() {
    let x = template(hyprtxt!(
        body {
            p {
                $: "this is some text"
                div {
                    $: "this is some nested text"
                }
            }
        }
    ));
    dbg!(x);
}

fn template(slot: String) -> String {
    hyprtxt!(
        html {
            lang: "en"
            head {
                meta { charset: "UTF-8" }
                meta { name: "viewport", content: "width=device-width" }
                title { $: "Will this work???" }
            }
            $: slot
        }
    )
}

fn write_to_file(input: String) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("foo.html")?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
