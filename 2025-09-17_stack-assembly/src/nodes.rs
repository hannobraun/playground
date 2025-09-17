use std::{array, fmt, iter, vec};

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
    type IntoIter = iter::Chain<vec::IntoIter<Node>, array::IntoIter<Node, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter().chain([self.last])
    }
}

#[derive(Debug)]
pub enum Node {
    Comment { text: String },
    Empty,

    Assert,
    Equals,
    Nop,

    Integer { value: i32 },

    UnknownIdentifier { name: String },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comment { text } => write!(f, "#{text}"),
            Self::Empty => write!(f, ""),

            Self::Assert => write!(f, "assert"),
            Self::Equals => write!(f, "="),
            Self::Nop => write!(f, "nop"),

            Self::Integer { value } => write!(f, "{value}"),

            Self::UnknownIdentifier { name } => write!(f, "{name}"),
        }
    }
}
