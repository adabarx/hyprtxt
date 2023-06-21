use std::collections::HashMap;

use hyprtxt::hyprtxt;

pub fn page_template(slot: String) -> String {
    hyprtxt!(
        html {
            lang: "en"
            head {
                meta { charset: "UTF-8" }
                meta { name: "viewport", content: "width=device-width" }
                title { $: "Will this work???" }
                script { 
                    src: "https://unpkg.com/htmx.org@1.9.2"
                    integrity: "sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h"
                    crossorigin: "anonymous"
                }
            }
            $: slot
        }
    )
}

pub trait StaticSite {
    fn add_endpoint(&mut self, path: String, response: String);
    fn generate(&self) -> Result<(), std::io::Error>;
}

impl StaticSite for HashMap<String, String> {
    fn add_endpoint(&mut self, path: String, response: String) {
        let page = page_template(response);
        self.insert(path, page);
    }

    fn generate(&self) -> Result<(), std::io::Error> {
        use std::fs;

        for (path, page) in self.iter() {
            fs::write(["/public", path].join(""), page.to_string())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
