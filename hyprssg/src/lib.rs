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

#[derive(PartialEq, Eq, Hash)]
pub struct Endpoint {
    dir: Vec<String>,
    name: String,
}

impl Endpoint {
    pub fn new(mut path: Vec<String>) -> Self {
        let name = path.pop().unwrap();
        Self { dir: path, name }
    }

    pub fn dir(&self) -> String {
        let mut dir = "/".to_string();
        dir.push_str(self.dir.join("/").as_str());
        dir
    }

    pub fn name(&self) -> String {
        let mut name = self.name.clone();
        name.push_str(".html");
        name
    }

    pub fn path(&self) -> String {
        [self.dir(), self.name()].join("/")
    }
}

pub trait StaticSite {
    fn add_endpoint(&mut self, path: Endpoint, response: String);
    fn generate(&self) -> Result<(), std::io::Error>;
}

impl StaticSite for HashMap<Endpoint, String> {
    fn add_endpoint(&mut self, path: Endpoint, response: String) {
        let page = page_template(response);
        self.insert(path, page);
    }

    fn generate(&self) -> Result<(), std::io::Error> {
        use std::fs;

        for (path, page) in self.iter() {
            fs::write(path.path(), page.to_string())?;

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
