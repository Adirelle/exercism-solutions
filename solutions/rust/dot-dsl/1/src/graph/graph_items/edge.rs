#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge<'a> {
    pub source: &'a str,
    pub dest: &'a str,
}

impl<'a> Edge<'a> {
    pub fn new(source: &'a str, dest: &'a str) -> Self {
        Self { source, dest }
    }
}
