pub type Nodes = Vec<Node>;

pub enum Node {
    Assert,
    Equals,
    Integer { value: i32 },

    UnknownIdentifier,
}
