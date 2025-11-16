use std::fs::File;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Keywords
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // operators
    Add,       // +
    Sub,       // -
    Mul,       // *
    Div,       // /
    Mod,       // %
    Pow,       // ^
    Len,       // #
    BitAnd,    // &
    BitXor,    // ~
    BitOr,     // |
    ShiftL,    // <<
    ShiftR,    // >>
    Idiv,      // //
    Equal,     // ==
    NotEq,     // ~=
    LesEq,     // <=
    GreEq,     // >=
    Less,      // <
    Greater,   // >
    Assign,    // =
    ParL,      // (
    ParR,      // )
    CurlyL,    // {
    CurlyR,    // }
    SqurL,     // [
    SqurR,     // ]
    DoubColon, // ::
    SemiColon, // ;
    Colon,     // :
    Comma,     // ,
    Dot,       // .
    Concat,    // ..
    Dots,      // ...

    Name(&'a str),
    String(&'a str),
    Integer(i64),
    Float(f64),
    Eof,
}

#[derive(Debug)]
pub struct Lex<'a> {
    input: &'a str,
    pos: usize,
    line_number: u32,
    line_pos_offset: usize,
}

impl<'a> Lex<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line_number: 1,
            line_pos_offset: 0,
        }
    }

    pub fn next(&mut self) -> Token<'a> {
        while let Some(b) = self.peek_byte() {
            match b {
                b' ' | b'\t' => self.pos += 1,
                b'\r' | b'\n' => self.next_line(),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => return self.lex_identifier(),
                b'0'..=b'9' => return self.lex_number(),
                b'"' => return Token::String(self.lex_string()),
                _ => return self.lex_operator(),
            }
        }
        Token::Eof
    }

    pub fn line_number(&self) -> u32 {
        self.line_number
    }

    pub fn line_position(&self) -> usize {
        self.pos - self.line_pos_offset + 1
    }

    fn next_line(&mut self) {
        self.pos += 1;
        self.line_number += 1;
        self.line_pos_offset = self.pos;
    }

    fn peek_byte(&self) -> Option<u8> {
        if self.pos < self.input.len() {
            Some(self.input.as_bytes()[self.pos])
        } else {
            None
        }
    }

    fn lex_identifier(&mut self) -> Token<'a> {
        let start = self.pos;
        while let Some(b) = self.peek_byte() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let ident = &self.input[start..self.pos];

        return match ident {
            "and" => Token::And,
            "break" => Token::Break,
            "do" => Token::Do,
            "else" => Token::Else,
            "elseif" => Token::Elseif,
            "end" => Token::End,
            "false" => Token::False,
            "for" => Token::For,
            "function" => Token::Function,
            "goto" => Token::Goto,
            "if" => Token::If,
            "in" => Token::In,
            "local" => Token::Local,
            "nil" => Token::Nil,
            "not" => Token::Not,
            "or" => Token::Or,
            "repeat" => Token::Repeat,
            "return" => Token::Return,
            "then" => Token::Then,
            "true" => Token::True,
            "Until" => Token::Until,
            "while" => Token::While,
            _ => Token::Name(ident),
        };
    }

    fn lex_operator(&mut self) -> Token<'a> {
        let b = self.peek_byte().unwrap();
        self.pos += 1;

        match b {
            b'+' => Token::Add,
            b'-' => Token::Sub,
            b'*' => Token::Mul,
            b'/' => {
                if self.peek_byte() == Some(b'/') {
                    self.pos += 1;
                    Token::Idiv
                } else {
                    Token::Div
                }
            }
            b'=' => {
                if self.peek_byte() == Some(b'=') {
                    self.pos += 1;
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'~' => {
                if self.peek_byte() == Some(b'=') {
                    self.pos += 1;
                    Token::NotEq
                } else {
                    Token::BitXor
                }
            }
            b'.' => {
                if self.peek_byte() == Some(b'.') {
                    self.pos += 1;
                    if self.peek_byte() == Some(b'.') {
                        self.pos += 1;
                        Token::Dots // ...
                    } else {
                        Token::Concat // ..
                    }
                } else {
                    Token::Dot
                }
            }
            b'<' => {
                if self.peek_byte() == Some(b'=') {
                    self.pos += 1;
                    Token::LesEq
                } else {
                    Token::Less
                }
            }
            b'>' => {
                if self.peek_byte() == Some(b'=') {
                    self.pos += 1;
                    Token::GreEq
                } else {
                    Token::Greater
                }
            }
            b'&' => Token::BitAnd,
            b'|' => Token::BitOr,
            b'^' => Token::BitXor,
            b'#' => Token::Len,
            b'(' => Token::ParL,
            b')' => Token::ParR,
            b'{' => Token::CurlyL,
            b'}' => Token::CurlyR,
            b'[' => Token::SqurL,
            b']' => Token::SqurR,
            b':' => {
                if self.peek_byte() == Some(b':') {
                    self.pos += 1;
                    Token::DoubColon
                } else {
                    Token::Colon
                }
            }
            b';' => Token::SemiColon,
            b',' => Token::Comma,
            _ => panic!("Unknown operator: {}", b as char),
        }
    }

    fn lex_number(&mut self) -> Token<'a> {
        let start = self.pos;
        let mut has_dot = false;
        let mut has_exp = false;

        while let Some(b) = self.peek_byte() {
            match b {
                b'0'..=b'9' => self.pos += 1,
                b'.' if !has_dot && !has_exp => {
                    has_dot = true;
                    self.pos += 1;
                }
                b'e' | b'E' if !has_exp => {
                    has_exp = true;
                    self.pos += 1;
                    if let Some(b'+' | b'-') = self.peek_byte() {
                        self.pos += 1;
                    }
                }
                _ => break,
            }
        }

        let slice = &self.input[start..self.pos];

        match has_dot || has_exp {
            true => Token::Float(slice.parse().unwrap()),
            false => Token::Integer(slice.parse().unwrap()),
        }
    }

    fn lex_string(&mut self) -> &'a str {
        self.pos += 1; // skip opening quote
        let start = self.pos;
        while let Some(b) = self.peek_byte() {
            if b == b'"' {
                break;
            }
            self.pos += 1;
        }
        let s = &self.input[start..self.pos];
        self.pos += 1; // skip closing quote
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_numbers() {
        let mut lex = Lex::new("123 4.56 444 4.55555555 4.57e-3 0.3e12 5e+20");
        assert_eq!(lex.next(), Token::Integer(123));
        assert_eq!(lex.next(), Token::Float(4.56));
        assert_eq!(lex.next(), Token::Integer(444));
        assert_eq!(lex.next(), Token::Float(4.55555555));
        assert_eq!(lex.next(), Token::Float(4.57e-3));
        assert_eq!(lex.next(), Token::Float(0.3e12));
        assert_eq!(lex.next(), Token::Float(5e+20));
        assert_eq!(lex.next(), Token::Eof);
    }

    #[test]
    fn lex_identifiers_and_keywords() {
        let mut lex = Lex::new("if x then end foo_bar");
        assert_eq!(lex.next(), Token::If);
        assert_eq!(lex.next(), Token::Name("x"));
        assert_eq!(lex.next(), Token::Then);
        assert_eq!(lex.next(), Token::End);
        assert_eq!(lex.next(), Token::Name("foo_bar"));
        assert_eq!(lex.next(), Token::Eof);
    }
}
