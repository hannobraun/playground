pub fn parse() -> Program {
    Program {
        function: Function {
            name: "start",
            value: 42,
        },
    }
}

pub struct Program {
    pub function: Function,
}

pub struct Function {
    pub name: &'static str,
    pub value: i32,
}
