pub struct Nodes {
    pub inner: Vec<Node>,
}

impl Nodes {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }
}

pub enum Node {
    Assert,
    Equals,
    Integer { value: i32 },

    UnknownIdentifier,
}
