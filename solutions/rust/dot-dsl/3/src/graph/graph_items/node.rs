use std::collections::HashMap;

use crate::graph::attrs::Attrs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        Self {
            name: self.name.clone(),
            attrs: self.attrs.with_attrs(attrs),
        }
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.attr(name)
    }
}
