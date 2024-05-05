use html_parser::Node;

use crate::traits::{form_nodes::FromNodes, into_elements::IntoElements};

use self::{container_direction::ContainerDirection, container_expand::ContainerExpand, attributes::GetAttributes};

pub mod container_expand;
pub mod container_direction;
mod attributes;

#[derive(Debug, Clone)]
pub enum Element {
    Window {
        childs: Vec<Element>,
        direction: ContainerDirection,
        gap: u16,
        margin: u16,
        padding: u16,
    },
    Container {
        childs: Vec<Element>,
        expand: Option<ContainerExpand>,
        direction: ContainerDirection,
        gap: u16,
        margin: u16,
        padding: u16,
    },
    Button {
        childs: Vec<Element>,
        expand: Option<ContainerExpand>,
        direction: ContainerDirection,
        gap: u16,
        margin: u16,
        padding: u16,
    },
    Text {
        text: String,
        font_size: u8,
        margin: u16,
        padding: u16,
    },
    Space,
}

impl TryFrom<Node> for Element {
    type Error = String;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let Node::Element(element) = node else {
            return Err(String::new());
        };
        match element.name.as_str() {
            "window" => Ok(Self::Window {
                childs: element.children.into_elements(),
                direction: element.attributes.direction(),
                gap: element.attributes.gap(),
                margin: element.attributes.margin(),
                padding: element.attributes.padding(),
            }),
            "container" => Ok(Self::Container {
                childs: element.children.into_elements(),
                expand: element.attributes.expand(),
                direction: element.attributes.direction(),
                gap: element.attributes.gap(),
                margin: element.attributes.margin(),
                padding: element.attributes.padding(),
            }),
            "button" => Ok(Self::Button {
                childs: element.children.into_elements(),
                expand: element.attributes.expand(),
                direction: element.attributes.direction(),
                gap: element.attributes.gap(),
                margin: element.attributes.margin(),
                padding: element.attributes.padding(),
            }),
            "text" => Ok(Self::Text {
                text: String::from_nodes(element.children),
                font_size: element.attributes.font_size(),
                margin: element.attributes.margin(),
                padding: element.attributes.padding(),
            }),
            "space" => Ok(Self::Space),
            &_ => Err(String::new()),
        }
    }
}