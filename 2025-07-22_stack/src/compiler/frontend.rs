pub fn parse() -> Program {
    Program {
        function: "start",
        value: 42,
    }
}

pub struct Program {
    pub function: &'static str,
    pub value: i32,
}
