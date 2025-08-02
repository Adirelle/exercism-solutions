#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node<'a> {
    pub name: &'a str,
}

impl<'a> Node<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name: name }
    }
}
