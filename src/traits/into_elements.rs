use html_parser::Node;
use crate::element::Element;

pub trait IntoElements<T> {
    fn into_elements(self) -> Vec<Element>;
}

impl IntoElements<Vec<Node>> for Vec<Node> {
    fn into_elements(self) -> Vec<Element> {
        self.iter()
            .filter_map(|n| Element::try_from(n.clone()).ok())
            .collect()
    }
}