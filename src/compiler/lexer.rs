#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Semicolon,
    Equals,
    Identifier(String),
    Let,
    If,
    Else,
    While,
    Print,
    DoubleEquals,
    LessThan,
    GreaterThan,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(ch) = self.peek() {
            if !ch.is_digit(10) {
                break;
            }
            number.push(ch);
            self.advance();
        }
        Token::Number(number.parse().unwrap())
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            ident.push(ch);
            self.advance();
        }
        
        match ident.as_str() {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "print" => Token::Print,
            _ => Token::Identifier(ident),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        
        let ch = self.peek()?;
        match ch {
            '0'..='9' => Some(self.read_number()),
            'a'..='z' | 'A'..='Z' | '_' => Some(self.read_identifier()),
            '+' => { self.advance(); Some(Token::Plus) },
            '-' => { self.advance(); Some(Token::Minus) },
            '*' => { self.advance(); Some(Token::Star) },
            '/' => { self.advance(); Some(Token::Slash) },
            '(' => { self.advance(); Some(Token::LParen) },
            ')' => { self.advance(); Some(Token::RParen) },
            ';' => { self.advance(); Some(Token::Semicolon) },
            '=' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Some(Token::DoubleEquals)
                } else {
                    Some(Token::Equals)
                }
            },
            '<' => { self.advance(); Some(Token::LessThan) },
            '>' => { self.advance(); Some(Token::GreaterThan) },
            _ => None,
        }
    }
}