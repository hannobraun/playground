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
            last: Node::Empty,
        }
    }
}

impl IntoIterator for Nodes {
    type Item = Node;
    type IntoIter = iter::Chain<vec::IntoIter<Node>, option::IntoIter<Node>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter().chain(Some(self.last))
    }
}

#[derive(Debug)]
pub enum Node {
    Assert,
    Equals,
    Nop,

    Integer { value: i32 },

    Empty,
    UnknownIdentifier { name: String },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assert => write!(f, "assert"),
            Self::Equals => write!(f, "="),
            Self::Nop => write!(f, "nop"),

            Self::Integer { value } => write!(f, "{value}"),

            Self::Empty => write!(f, ""),
            Self::UnknownIdentifier { name } => write!(f, "{name}"),
        }
    }
}
