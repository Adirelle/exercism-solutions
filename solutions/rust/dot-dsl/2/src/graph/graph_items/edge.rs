use std::collections::HashMap;

use crate::graph::attrs::Attrs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub source: String,
    pub dest: String,
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(source: &str, dest: &str) -> Self {
        Self {
            source: source.into(),
            dest: dest.into(),
            attrs: HashMap::new(),
        }
    }
}

impl Attrs for Edge {
    fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        Self {
            attrs: self.attrs.with_attrs(attrs),
            source: self.source.clone(),
            dest: self.dest.clone(),
        }
    }

    fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.attr(name)
    }
}
