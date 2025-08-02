use std::collections::HashMap;

use self::{
    attrs::Attrs,
    graph_items::{edge::Edge, node::Node},
};

mod attrs;
pub mod graph_items;

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_edges(&self, edges: &Vec<Edge>) -> Self {
        Self {
            edges: self.edges.with(edges),
            nodes: self.nodes.clone(),
            attrs: self.attrs.clone(),
        }
    }

    pub fn with_nodes(&self, nodes: &Vec<Node>) -> Self {
        Self {
            nodes: self.nodes.with(nodes),
            edges: self.edges.clone(),
            attrs: self.attrs.clone(),
        }
    }

    pub fn node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }

    pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        Self {
            attrs: self.attrs.with_attrs(attrs),
            edges: self.edges.clone(),
            nodes: self.nodes.clone(),
        }
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.attr(name)
    }
}

trait VecWith<T> {
    fn with(&self, items: &Vec<T>) -> Self;
}

impl<T: Clone> VecWith<T> for Vec<T> {
    fn with(&self, items: &Vec<T>) -> Self {
        let mut new = self.clone();
        for item in items {
            new.push(item.clone());
        }
        new
    }
}
