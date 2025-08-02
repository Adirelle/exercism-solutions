use std::collections::HashMap;

use self::graph_items::{edge::Edge, node::Node};

pub mod graph_items;

#[derive(Debug, Clone)]
pub struct Graph<'a> {
    pub nodes: Vec<Node<'a>>,
    pub edges: Vec<Edge<'a>>,
    pub attrs: HashMap<&'a str, &'a str>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_edges(&self, edges: &'a Vec<Edge>) -> Self {
        let mut m = self.edges.clone();
        for e in edges {
            m.push(*e);
        }
        Self {
            edges: m,
            nodes: self.nodes.clone(),
            attrs: self.attrs.clone(),
        }
    }

    pub fn with_nodes(&self, nodes: &'a Vec<Node>) -> Self {
        let mut m = self.nodes.clone();
        for n in nodes {
            m.push(*n);
        }
        Self {
            nodes: m,
            edges: self.edges.clone(),
            attrs: self.attrs.clone(),
        }
    }
}
