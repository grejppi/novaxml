pub mod nodes;
pub mod parser;
pub mod tokenizer;

mod tests;

use tokenizer::*;
use parser::*;
use nodes::*;

pub struct Document {
    root: Element,
}

impl Document {
    pub fn new() -> Document {
        Document {
            root: Element::new("root"),
        }
    }

    pub fn from_element(e: Element) -> Document {
        Document {
            root: e,
        }
    }

    pub fn from_string(s: &str) -> Result<Document, String> {
        let tokens = match tokenize(&s) {
            Ok(tokens) => tokens,
            Err(e) => return Err(e),
        };

        let element = match parse(tokens) {
            Ok(element) => element,
            Err(e) => return Err(e),
        };

        Ok(Document::from_element(element))
    }

    pub fn from_file(p: &str) -> Result<Document, String> {
        let string = match string_from_file(p) {
            Some(string) => string,
            None => return Err("Couldn't make String from file".into()),
        };
            
        match Document::from_string(&string) {
            Ok(document) => Ok(document),
            Err(e) => Err(e),
        }
    }

    pub fn get_root(&self) -> &Element {
        match self.root.get_first_child() {
            Some(c) => c,
            None => panic!("Document has no root element!"),
        }
    }

    pub fn print(&self) {
        self.root.print(0);
    }
}
