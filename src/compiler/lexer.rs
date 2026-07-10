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
    LessEqual,
    GreaterEqual,
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
            if !ch.is_ascii_digit() {
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
            if !ch.is_ascii_alphanumeric() && ch != '_' {
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
            '+' => {
                self.advance();
                Some(Token::Plus)
            }
            '-' => {
                self.advance();
                Some(Token::Minus)
            }
            '*' => {
                self.advance();
                Some(Token::Star)
            }
            '/' => {
                self.advance();
                Some(Token::Slash)
            }
            '(' => {
                self.advance();
                Some(Token::LParen)
            }
            ')' => {
                self.advance();
                Some(Token::RParen)
            }
            ';' => {
                self.advance();
                Some(Token::Semicolon)
            }
            '=' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Some(Token::DoubleEquals)
                } else {
                    Some(Token::Equals)
                }
            }
            '<' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Some(Token::LessEqual)
                } else {
                    Some(Token::LessThan)
                }
            }
            '>' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::GreaterThan)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    fn collect_tokens(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        tokens
    }

    #[test]
    fn tokenizes_single_character_comparisons() {
        assert_eq!(
            collect_tokens("x < 10; y > 3;"),
            vec![
                Token::Identifier("x".to_string()),
                Token::LessThan,
                Token::Number(10),
                Token::Semicolon,
                Token::Identifier("y".to_string()),
                Token::GreaterThan,
                Token::Number(3),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn tokenizes_inclusive_comparisons() {
        assert_eq!(
            collect_tokens("x <= 10; y >= 3;"),
            vec![
                Token::Identifier("x".to_string()),
                Token::LessEqual,
                Token::Number(10),
                Token::Semicolon,
                Token::Identifier("y".to_string()),
                Token::GreaterEqual,
                Token::Number(3),
                Token::Semicolon,
            ]
        );
    }
}
