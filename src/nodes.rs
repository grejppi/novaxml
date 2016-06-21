use std::fmt;
use std::slice;
use std::iter::Cloned;

#[derive(Clone)]
pub struct Element {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Element>,
    empty: bool,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}\n\tattr: {}\n\tchildren: {}\n\tempty: {}",
               self.name,
               self.attributes.len(),
               self.children.len(),
               self.empty)
    }
}

impl Element {
    pub fn new_empty() -> Element {
        Element {
            name: String::new(),
            attributes: Vec::new(),
            children: Vec::new(),
            empty: false,
        }
    }
    pub fn new(s: &str) -> Element {
        Element {
            name: s.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
            empty: false,
        }
    }

    pub fn append_child(&mut self, e: Element) {
        self.children.push(e);
    }
    
    pub fn append_attribute(&mut self, a: Attribute) {
        self.attributes.push(a);
    }

    pub fn set_text(&mut self, t: &str) {
        self.attributes.push(Attribute::new("text_data", t));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn set_name(&mut self, s: &str) {
        self.name = s.to_string();
    }

    pub fn get_child_count(&self) -> usize {
        self.children.len()
    }

    pub fn set_empty(&mut self, b: bool) {
        self.empty = b;
    }

    pub fn is_empty(&mut self) -> bool {
        self.empty
    }

    pub fn set_attributes(&mut self, a: Vec<Attribute>) {
        self.attributes = a;
    }

    pub fn print(&self, indent: usize) {
        let mut s = String::new();
        for _ in 0..indent {
            s.push('\t')
        }

        let mut attr = String::new();
        for a in &self.attributes {
            attr.push(' ');
            attr.push_str(&a.print());
        }

        if self.empty == true {
            println!("{}<{}{}/>", s, self.name, attr);
            return;
        }

        println!("{}<{}{}>", s, self.name, attr);
        for e in &self.children {
            e.print(indent + 1);
        }
        println!("{}</{}>", s, self.name);
    }

    // methods for accessing element content
    pub fn get_child(&self, name: &str) -> Option<&Element> {
        self.children.iter().by_ref().find(|ref x| x.get_name() == name)
    }

    pub fn get_first_child(&self) -> Option<&Element> {
        self.children.first()
    }

    pub fn iter_children(&self) -> Cloned<slice::Iter<Element>> {
        // TODO: find a better way to do this
        self.children.iter().cloned()
    }

    pub fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.iter().by_ref().find(|ref x| x.get_name() == name)
    }

    pub fn iter_attributes(&self) -> Cloned<slice::Iter<Attribute>> {
        // TODO: find a better way to do this
        self.attributes.iter().cloned()
    }
}

#[derive(Clone, Debug)]
pub struct Attribute {
    name: String,
    value: String,
}

impl Attribute {
    pub fn new(n: &str,
               v: &str) -> Attribute {
        Attribute {
            name: n.to_string(),
            value: v.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn print(&self) -> String {
        format!("{}=\"{}\"", self.name, self.value)
    }
}
