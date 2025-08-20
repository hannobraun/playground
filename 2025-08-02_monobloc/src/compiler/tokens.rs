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

    pub fn process_char(&mut self, ch: char) -> ProcessCharOutcome {
        match (&self.state, ch) {
            (State::Initial, '#') => {
                self.state = State::Comment;
            }
            (State::Initial, ch) if ch.is_whitespace() => {
                let buf = mem::take(&mut self.buf);

                if buf.is_empty() {
                    return ProcessCharOutcome::TokenNotReady;
                }

                let token = if let Ok(value) = buf.parse() {
                    Token::Number { value }
                } else {
                    Token::Identifier { name: buf }
                };

                return ProcessCharOutcome::TokenIsReady { token };
            }
            (State::Initial, ch) => {
                self.buf.push(ch);
            }
            (State::Comment, '\n') => {
                self.state = State::Initial;

                return ProcessCharOutcome::TokenIsReady {
                    token: Token::Comment {
                        text: mem::take(&mut self.buf),
                    },
                };
            }
            (State::Comment, ch) => {
                self.buf.push(ch);
            }
        }

        ProcessCharOutcome::TokenNotReady
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
    Number { value: i32 },
}

#[derive(Debug)]
pub enum ProcessCharOutcome {
    TokenIsReady { token: Token },
    TokenNotReady,
}
