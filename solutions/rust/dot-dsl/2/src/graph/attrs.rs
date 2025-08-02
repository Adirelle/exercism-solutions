use std::collections::HashMap;

pub trait Attrs {
    fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self;
    fn attr(&self, name: &str) -> Option<&str>;
}

impl Attrs for HashMap<String, String> {
    fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        let mut new = self.clone();
        for (k, v) in attrs {
            new.insert(String::from(*k), String::from(*v));
        }
        new
    }

    fn attr(&self, name: &str) -> Option<&str> {
        match self.get(name.into()) {
            Some(s) => Some(&*s),
            None => None,
        }
    }
}
