use std::{iter, option, vec};

pub struct Nodes {
    pub inner: Vec<Node>,
    pub last: Node,
}

impl Nodes {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            last: Node::UnknownIdentifier {
                name: String::new(),
            },
        }
    }
}

impl IntoIterator for Nodes {
    type Item = Node;
    type IntoIter = iter::Chain<vec::IntoIter<Node>, option::IntoIter<Node>>;

    fn into_iter(self) -> Self::IntoIter {
        let Node::UnknownIdentifier { name: identifier } = self.last else {
            unreachable!(
                "Only ever setting `self.last` to `UnknownIdentifier`."
            );
        };

        let last = if identifier.is_empty() {
            None
        } else {
            Some(Node::UnknownIdentifier { name: identifier })
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
