use html_parser::Node;
use raylib::RaylibHandle;
use serde_json::Value;

use crate::{format::{find_key, fmt}, loader::Loader};

use self::{attributes::GetAttributes, container_direction::ContainerDirection, container_expand::ContainerExpand, processed_element::ProcessedElement};

mod attributes;
pub mod container_direction;
pub mod container_expand;
pub mod processed_element;
pub mod prepared_element;
pub mod renderable_element;

#[derive(Debug, Clone)]
pub enum Element {
    Text(String),
    Element {
        childs: Vec<Element>,
        expand: Option<ContainerExpand>,
        direction: ContainerDirection,
        gap: u16,
        margin: u16,
        padding: u16,
        scrollable: Option<ContainerDirection>,
        font_size: Option<u8>,
    },
    For {
        each: String,
        display: Box<Element>,
    }
}

pub trait IntoElements<T> {
    fn into_elements(self, loader: &Loader) -> Vec<Element>;
}

impl IntoElements<Vec<Node>> for Vec<Node> {
    fn into_elements(self, loader: &Loader) -> Vec<Element> {
        self.iter()
            .filter_map(|n| n.clone().into_element(loader).ok())
            .collect()
    }
}

pub trait IntoElement<T> {
    type Error;
    fn into_element(self, loader: &Loader) -> Result<Element, Self::Error>;
}

impl IntoElement<Node> for Node {
    type Error = String;

    fn into_element(self, loader: &Loader) -> Result<Element, Self::Error> {
        match self {
            Self::Element(element) => {
                if element.name.as_str() == "for" {
                    return Ok(Element::For {
                        each: element.attributes.each(),
                        display: Box::new(loader.get(&element.attributes.display())),
                    });
                }

                Ok(Element::Element {
                    childs: element.children.into_elements(loader),
                    expand: element.attributes.expand(),
                    direction: element.attributes.direction(),
                    gap: element.attributes.gap(),
                    margin: element.attributes.margin(),
                    padding: element.attributes.padding(),
                    scrollable: element.attributes.scrollable(),
                    font_size: element.attributes.font_size(),
                })
            },
            Self::Text(str) => Ok(Element::Text(str)),
            Self::Comment(_) => Err(String::new()),
        }
    }
}

impl Element {

    pub fn expand(self, rl: &RaylibHandle, context: &Value) -> Vec<ProcessedElement> {
        match self {
            Self::Text(text) => vec![ ProcessedElement::Text(fmt(context, text)) ],
            Self::For { each, display } => {
                let each_ctx = find_key(context, &each);

                let Some(Value::Array(arr)) = each_ctx else {
                    panic!("{} is not an array", each);
                };

                arr.into_iter().flat_map(|value| display.clone().expand(rl, value)).collect()
            },
            Self::Element { childs, expand, direction, gap, margin, padding, scrollable, font_size } => {
                vec![ ProcessedElement::Element {
                    childs: childs.into_iter().flat_map(|child| child.expand(rl, context)).collect(),
                    expand, direction, gap, margin, padding, scrollable, font_size,
                } ]
            },
        }
    }

}
