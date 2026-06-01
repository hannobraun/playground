#[derive(Debug)]
pub enum Token {
    Binding,
    BlockClose,
    BlockOpen,
    Comment { text: String },
    Identifier { name: String },
    Integer { value: u32, format: IntegerFormat },
    Terminator,
}

#[derive(Debug)]
pub enum IntegerFormat {
    Hex,
    Signed,
    Unsigned,
}
