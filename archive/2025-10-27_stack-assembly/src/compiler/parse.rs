use crate::instructions::Operator;

pub fn parse_operator(token: &str) -> Option<Operator> {
    let operator = if token == "call" {
        Operator::Call
    } else if token == "call_if" {
        Operator::CallIf
    } else if token == "drop0" {
        Operator::Drop0
    } else if token == "yield" {
        Operator::Yield
    } else if let Ok(value) = token.parse() {
        Operator::Integer { value }
    } else {
        return None;
    };

    Some(operator)
}
