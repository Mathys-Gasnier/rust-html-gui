use std::error::Error;

use html_parser::Dom;
use serde::Serialize;

use crate::{element::Element, traits::into_elements::IntoElements};

mod consts;
mod traits;
mod element;
mod render;

#[derive(Serialize)]
pub struct Context {
    app_name: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let html = include_str!("../main.gui");

    let tree = Dom::parse(html)?;

    let elements: Vec<Element> = tree.children.into_elements();

    #[cfg(debug_assertions)]
    println!("{:#?}", elements);

    assert!(elements.len() == 1, "Can't have more than one top element");
    assert!(matches!(elements[0], Element::Window { .. }), "Top element must be of type window");

    let window = elements[0].clone();
    let context = Context {
        app_name: String::from("Demo App"),
    };

    render::render(window, &context);

    Ok(())
}
