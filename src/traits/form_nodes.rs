use html_parser::Node;

pub trait FromNodes {
    fn from_nodes(nodes: Vec<Node>) -> Self;
}

impl FromNodes for String {
    fn from_nodes(nodes: Vec<Node>) -> Self {
        nodes.iter().filter_map(|n| n.text()).collect::<String>().trim().to_owned()
    }
}