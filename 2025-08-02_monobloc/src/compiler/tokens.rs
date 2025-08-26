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

                let token_as_u32: Option<u32> = buf.parse().ok();
                let token_as_i32: Option<i32> = buf.parse().ok();

                let token = if let Some(value) = token_as_u32 {
                    let value = i32::from_le_bytes(value.to_le_bytes());
                    Token::Integer { value }
                } else if let Some(value) = token_as_i32 {
                    Token::Integer { value }
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
    Comment { text: String },
    Identifier { name: String },
    Integer { value: i32 },
}
