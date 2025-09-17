pub struct Nodes {
    pub inner: Vec<Node>,
}

impl Nodes {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

pub enum Node {
    Assert,
    Equals,
    Integer { value: i32 },

    UnknownIdentifier,
}
