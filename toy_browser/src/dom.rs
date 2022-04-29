//! Basic DOM data structures.

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

/// AttributeMap keys are, for example, `id` and `class`.
/// e.g. {"id": "myHeader", "class": "city"}
pub type AttributeMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct ElementData {
    // e.g. "h1", "p", "div"
    pub tag_name: String,
    pub attributes: AttributeMap,
}

// Constructor functions for convenience:
pub fn text(data: String) -> Node {
    Node {
        children: vec![],
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attributes: AttributeMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes,
        }),
    }
}

// Element methods
impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}
