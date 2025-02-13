#[derive(Debug)]
pub enum Expr {
    Number(i64),
    BinaryOp(Box<Expr>, BinaryOpKind, Box<Expr>),
    Variable(String),
}

#[derive(Debug)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Equals,
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expr),
    Assign(String, Expr),
    If(Expr, Vec<Statement>, Vec<Statement>),
    While(Expr, Vec<Statement>),
    Print(Expr),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == Some(expected.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current_token))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while self.current_token.is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.current_token {
            Some(Token::Let) => {
                self.advance();
                if let Some(Token::Identifier(name)) = self.current_token.clone() {
                    self.advance();
                    self.expect(Token::Equals)?;
                    let expr = self.parse_expression()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Statement::Let(name, expr))
                } else {
                    Err("Expected identifier after 'let'".to_string())
                }
            },
            Some(Token::If) => {
                self.advance();
                let condition = self.parse_expression()?;
                let then_block = self.parse_block()?;
                let else_block = if self.current_token == Some(Token::Else) {
                    self.advance();
                    self.parse_block()?
                } else {
                    Vec::new()
                };
                Ok(Statement::If(condition, then_block, else_block))
            },
            Some(Token::While) => {
                self.advance();
                let condition = self.parse_expression()?;
                let block = self.parse_block()?;
                Ok(Statement::While(condition, block))
            },
            Some(Token::Print) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Statement::Print(expr))
            },
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                self.expect(Token::Equals)?;
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Statement::Assign(name, expr))
            },
            _ => Err("Expected statement".to_string()),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        
        match &self.current_token {
            Some(Token::LParen) => {
                self.advance();
                while self.current_token.is_some() && self.current_token != Some(Token::RParen) {
                    statements.push(self.parse_statement()?);
                }
                self.expect(Token::RParen)?;
            }
            _ => {
                statements.push(self.parse_statement()?);
            }
        }
        
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_additive()?;

        while let Some(token) = &self.current_token {
            let op = match token {
                Token::DoubleEquals => BinaryOpKind::Equals,
                Token::LessThan => BinaryOpKind::LessThan,
                Token::GreaterThan => BinaryOpKind::GreaterThan,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplicative()?;

        while let Some(token) = &self.current_token {
            let op = match token {
                Token::Plus => BinaryOpKind::Add,
                Token::Minus => BinaryOpKind::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        while let Some(token) = &self.current_token {
            let op = match token {
                Token::Star => BinaryOpKind::Mul,
                Token::Slash => BinaryOpKind::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_primary()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current_token {
            Some(Token::Number(n)) => {
                let n = *n;
                self.advance();
                Ok(Expr::Number(n))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Variable(name))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err("Expected expression".to_string()),
        }
    }
}