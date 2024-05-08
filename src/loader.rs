use std::fs;

use html_parser::Dom;

use crate::element::{Element, IntoElements};


pub struct Loader {
    root: String
}

impl Loader {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string()
        }
    }

    pub fn get(&self, file: &str) -> Element {
        
        let html = fs::read_to_string(format!("{}/{}", self.root, file)).expect(&format!("Couldn't find `{}` in `{}`", file, self.root));
    
        let tree = Dom::parse(&html).expect("Couldn't parse gui file");
    
        let elements: Vec<Element> = tree.children.into_elements(self);
    
        #[cfg(debug_assertions)]
        println!("{:#?}", elements);
    
        assert!(elements.len() == 1, "Can't have more than one root element");

        elements[0].clone()
    }
}