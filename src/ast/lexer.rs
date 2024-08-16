use std::iter::Peekable;

pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    EOF,
}

pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos > self.input.len() {
            return None;
        }
        if self.current_pos == self.input.len() {
            let eof_char: char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(0, 0, eof_char.to_string()),
            ));
        }

        let c = self.current_char();
        if Self::is_number_start(&{
            self.tokenize_number()
        }
    }

        
    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }
    fn current_char(&self) -> char {
        self.input.chars().nth(self.current_pos).unwrap()
    }
}
