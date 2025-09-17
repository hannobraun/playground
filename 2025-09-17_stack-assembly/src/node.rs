use std::{fmt, iter, option, vec};

#[derive(Debug)]
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
        let last = if self.last.to_string().is_empty() {
            None
        } else {
            Some(self.last)
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assert => write!(f, "assert"),
            Self::Equals => write!(f, "="),
            Self::Integer { value } => write!(f, "{value}"),

            Self::UnknownIdentifier { name } => write!(f, "{name}"),
        }
    }
}
