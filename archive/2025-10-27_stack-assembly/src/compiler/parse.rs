use crate::instructions::Operator;

pub fn parse_label(token: &str) -> Option<&str> {
    if let Some((label, "")) = token.rsplit_once(":") {
        Some(label)
    } else {
        None
    }
}

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

pub fn parse_reference(token: &str) -> Option<&str> {
    if let Some(("", reference)) = token.split_once("@") {
        Some(reference)
    } else {
        None
    }
}
