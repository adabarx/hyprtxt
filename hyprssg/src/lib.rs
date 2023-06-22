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
    pub fn new(path: &[&str]) -> Self {
        let mut path = path.into_iter().peekable();
        let mut dir = vec![];
        let mut name = "".to_string();
        
        loop {
            if let Some(&next) = path.next() {
                if path.peek().is_some() {
                    dir.push(next.to_string());
                } else {
                    name.push_str(next);
                    break
                }
            }
        }

        Self { dir, name }
    }

    pub fn dir(&self, prefix: &str) -> String {
        [prefix.to_string(), "/".to_string(), self.dir.join("/")].join("")
    }

    pub fn name(&self) -> String {
        let mut name = self.name.clone();
        name.push_str(".html");
        name
    }

    pub fn path(&self, prefix: &str) -> String {
        [self.dir(prefix), self.name()].join("/")
    }
}

impl From<&str> for Endpoint {
    fn from(value: &str) -> Self {
        let mut dir: Vec<String> = value.split("/")
            .map(|name| name.to_string())
            .collect();
        let name = dir.pop().unwrap();

        Self { dir, name }
    }
}

pub trait StaticSite {
    fn add_endpoint(&mut self, path: Endpoint, response: String);
    fn generate(&self) -> Result<(), std::io::Error>;
}

impl StaticSite for HashMap<Endpoint, String> {
    fn add_endpoint(&mut self, path: Endpoint, response: String) {
        self.insert(path, response);
    }

    fn generate(&self) -> Result<(), std::io::Error> {
        use std::fs;
        let prefix = "public";

        for (endpoint, page) in self.iter() {
            fs::create_dir_all(endpoint.dir(prefix))?;
            fs::write(endpoint.path(prefix), page.to_string())?;

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
