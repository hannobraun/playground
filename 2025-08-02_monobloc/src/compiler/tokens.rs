use std::mem;

pub struct Tokenizer {
    state: State,
    buf: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            state: State::Initial,
            buf: String::new(),
        }
    }

    pub fn process_char(&mut self, ch: char) -> Option<Token> {
        match (&self.state, ch) {
            (State::Initial, '#') => {
                self.state = State::Comment;
            }
            (State::Initial, ch) if ch.is_whitespace() => {
                let buf = mem::take(&mut self.buf);

                if buf.is_empty() {
                    return None;
                }

                let token_as_unsigned_int: Option<u32> = buf.parse().ok();
                let token_as_signed_int: Option<i32> = buf.parse().ok();
                let token_as_hex_int = buf
                    .strip_prefix("0x")
                    .and_then(|s| u32::from_str_radix(s, 16).ok());

                let token = if let Some(value) = token_as_unsigned_int {
                    Token::Integer {
                        value,
                        format: IntegerFormat::Unsigned,
                    }
                } else if let Some(value) = token_as_signed_int {
                    let value = u32::from_le_bytes(value.to_le_bytes());

                    Token::Integer {
                        value,
                        format: IntegerFormat::Signed,
                    }
                } else if let Some(value) = token_as_hex_int {
                    Token::Integer {
                        value,
                        format: IntegerFormat::Hex,
                    }
                } else if buf == "=>" {
                    Token::Assignment
                } else {
                    Token::Identifier { name: buf }
                };

                return Some(token);
            }
            (State::Initial, ch) => {
                self.buf.push(ch);
            }
            (State::Comment, '\n') => {
                self.state = State::Initial;

                return Some(Token::Comment {
                    text: mem::take(&mut self.buf),
                });
            }
            (State::Comment, ch) => {
                self.buf.push(ch);
            }
        }

        None
    }
}

pub enum State {
    Initial,
    Comment,
}

#[derive(Debug)]
pub enum Token {
    Assignment,
    Comment { text: String },
    Identifier { name: String },
    Integer { value: u32, format: IntegerFormat },
}

#[derive(Debug)]
pub enum IntegerFormat {
    Hex,
    Signed,
    Unsigned,
}
