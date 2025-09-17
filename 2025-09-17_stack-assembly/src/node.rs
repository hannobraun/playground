use std::{iter, option, vec};

pub struct Nodes {
    pub inner: Vec<Node>,
    pub last: String,
}

impl Nodes {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            last: String::new(),
        }
    }
}

impl IntoIterator for Nodes {
    type Item = Node;
    type IntoIter = iter::Chain<vec::IntoIter<Node>, option::IntoIter<Node>>;

    fn into_iter(self) -> Self::IntoIter {
        let last = if self.last.is_empty() {
            None
        } else {
            Some(Node::UnknownIdentifier { name: self.last })
        };

        self.inner.into_iter().chain(last)
    }
}

#[derive(Debug)]
pub enum Node {
    Assert,
    Equals,
    Integer { value: i32 },

    UnknownIdentifier { name: String },
}
