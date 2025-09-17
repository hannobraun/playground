pub enum Node {
    Assert,
    Equals,
    Integer { value: i32 },

    UnknownIdentifier,
}
