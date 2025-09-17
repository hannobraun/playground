use std::vec;

pub struct Nodes {
    pub inner: Vec<Node>,
}

impl Nodes {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }
}

impl IntoIterator for Nodes {
    type Item = Node;
    type IntoIter = vec::IntoIter<Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug)]
pub enum Node {
    Assert,
    Equals,
    Integer { value: i32 },

    UnknownIdentifier { name: String },
}
