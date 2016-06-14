pub mod nodes;
pub mod parser;
pub mod tokenizer;

mod tests;

use std::option::Option;
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

    pub fn from_file(p: &str) -> Option<Document> {
        let string = match string_from_file(p) {
            Some(string) => string,
            None => return None,
        };
            
        let tokens = match tokenize(&string) {
            Some(tokens) => tokens,
            None => return None,
        };

        let element = match parse(tokens) {
            Some(element) => element,
            None => return None,
        };

        Some(Document::from_element(element))
    }

    pub fn print(&self) {
        self.root.print(0);
    }
}
