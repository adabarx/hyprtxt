#![allow(unused)]
use std::collections::HashMap;
use std::ops::Deref;
use std::{io::Error, fs};

use hyprtxt::hyprtxt;

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

pub struct SiteBuilder {
    pages: HashMap<Endpoint, String>,
    endpoints: HashMap<Endpoint, String>,
    template: Box<dyn Fn(String) -> String>,
}

impl SiteBuilder {

    pub fn new(template: impl Fn(String) -> String + 'static) -> Self {
        Self {
            pages: HashMap::new(),
            endpoints: HashMap::new(),
            template: Box::new(template),
        }
    }
}

impl StaticSite for SiteBuilder {
    fn add_page(&mut self, path: impl Into<Endpoint>, response: String) {
        self.pages.insert(path.into(), response);
    }

    fn add_endpoint(&mut self, path: impl Into<Endpoint>, response: String) {
        self.endpoints.insert(path.into(), response);
    }

    fn generate(&self) -> Result<(), Error> {
        let prefix = "./public";

        for (endpoint, page) in self.pages.iter() {
            fs::create_dir_all(endpoint.dir(prefix))?;
            fs::write(endpoint.path(prefix), (self.template)(page.to_string()))?;
        }

        for (endpoint, partial) in self.endpoints.iter() {
            let prefix = [prefix, "hmi"].join("/");
            fs::create_dir_all(endpoint.dir(&prefix))?;
            fs::write(endpoint.path(&prefix), partial.to_string())?;
        }

        Ok(())
    }
}

pub trait StaticSite {
    fn add_endpoint(&mut self, path: impl Into<Endpoint>, response: String);
    fn add_page(&mut self, path: impl Into<Endpoint>, response: String);
    fn generate(&self) -> Result<(), std::io::Error>;
}

pub fn bundle_assets() -> Result<Vec<String>, Error> {
    let paths = fs::read_dir("./assets")?;
    let mut rv = vec![];
    for path in paths {
        if let Ok(p) = path {
            let file_name = p.path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            if let Some(ext) = file_name.split(".").last() {
                match ext {
                    "js" => {
                        let path = "./public/assets/js";
                        let to = [path, file_name.as_ref()].join("/");
                        fs::create_dir_all(path)?;
                        fs::copy(p.path(), &to)?;

                        let to = &to[9..].to_string();
                        rv.push(hyprtxt!(script { src=to.to_string() }));
                    },
                    "css" => {
                        let path = "./public/assets/css";
                        let to = [path, file_name.as_ref()].join("/");
                        fs::create_dir_all(path)?;
                        fs::copy(p.path(), &to)?;

                        let to = &to[9..].to_string();
                        rv.push(hyprtxt!(link* { rel="stylesheet" type="text/css" href=to.to_string() }));
                    },
                    "pdf" => {
                        let path = "./public/assets/docs";
                        let to = [path, file_name.as_ref()].join("/");
                        fs::create_dir_all(path)?;
                        fs::copy(p.path(), &to)?;
                    },
                    "svg" => {
                        let path = "./public/assets/vector";
                        let to = [path, file_name.as_ref()].join("/");
                        fs::create_dir_all(path)?;
                        fs::copy(p.path(), &to)?;
                    },
                    "jpg" | "jpeg" | "png" => {
                        let path = "./public/assets/img";
                        let to = [path, file_name.as_ref()].join("/");
                        fs::create_dir_all(path)?;
                        fs::copy(p.path(), &to)?;
                    },
                    "mp3" | "ogg" | "flac" | "wav" => {
                        let path = "./public/assets/sounds";
                        let to = [path, file_name.as_ref()].join("/");
                        fs::create_dir_all(path)?;
                        fs::copy(p.path(), &to)?;
                    }
                    _ => continue,
                }
            }
        }
    }
    Ok(rv)
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
