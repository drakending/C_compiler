use std::fmt::{Debug, Display};

#[derive(Debug,PartialEq,Clone)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    WhiteSpace,
    Eof,
    Bad,
}

impl Display for TokenKind{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Number(num) => write!(f,"Number({})",num),
            TokenKind::Plus => write!(f,"Plus"),
            TokenKind::Minus => write!(f,"Minus"),
            TokenKind::Asterisk => write!(f,"Asterisk"),
            TokenKind::Slash => write!(f,"Slash"),
            TokenKind::LeftParen => write!(f,"LeftParen"),
            TokenKind::RightParen => write!(f,"RightParen"),
            TokenKind::WhiteSpace => write!(f,"WhiteSpace"),
            TokenKind::Eof => write!(f,"Eof"),
            TokenKind::Bad => write!(f,"Bad"),
        }
    }
}
#[derive(Debug,PartialEq,Clone)]
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

#[derive(Debug,Clone,PartialEq)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
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
        if self.current_pos == self.input.len() {
            let eof_char: char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, eof_char.to_string()),
            ));
        }
        let c = self.current_char();
        c.map(|c|{
            let start = self.current_pos;
            let mut kind = TokenKind::Bad;
            if Self::is_number_start(&c) {
                let number = self.consumer_number();
                kind = TokenKind::Number(number);
            } else if Self::is_whitespace(&c){
                self.consumer_whitespace();
                kind= TokenKind::WhiteSpace;
            }
            else {
                kind = self.consumer_punctuation();
            }
            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        })

    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;
        c
    }
    fn consumer_punctuation(&mut self) -> TokenKind{
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _   => TokenKind::Bad,
        }
    }
    fn consumer_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
    fn consumer_whitespace(&mut self){
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }
    fn is_whitespace(c: &char) -> bool { c.is_whitespace() }
    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }
}
