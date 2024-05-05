use std::error::Error;

use html_parser::Dom;

use crate::{element::Element, traits::into_elements::IntoElements};

mod consts;
mod traits;
mod element;
mod render;

fn main() -> Result<(), Box<dyn Error>> {
    let html = include_str!("../main.gui");

    let tree = Dom::parse(html)?;

    let elements: Vec<Element> = tree.children.into_elements();

    #[cfg(debug_assertions)]
    println!("{:#?}", elements);

    assert!(elements.len() == 1, "Can't have more than one top element");
    assert!(matches!(elements[0], Element::Window { .. }), "Top element must be of type window");

    let window = elements[0].clone();

    render::render(window);

    Ok(())
}
