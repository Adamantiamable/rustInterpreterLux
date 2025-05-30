
use crate::tool::generate_ast::{Expr, LiteralValue};
use crate::lox::token::{Token, TokenType};
use crate::lox::error_manager::ErrorManager;

#[derive(Debug, Clone)]
pub struct ParseError {
    token: Token,
    message: String,
}

impl ParseError {
    pub fn new(token: Token, message: &str) -> Self {
        ParseError {
            token,
            message: message.to_string(),
        }
    }
}
pub struct Parser <'a>
{
    tokens: Vec<Token>,
    current: usize,
    error_manager: &'a mut ErrorManager,
}

impl <'a> Parser <'a>{
    pub fn new(tokens: Vec<Token>, error_manager: &'a mut ErrorManager) -> Self {
        Parser {tokens, current:0, error_manager}
    }

    fn error(& mut self, token: &Token, message: &str)-> Result<(), ParseError> {
        self.error_manager.report(token.line, message, Some(&token.lexeme));
        Err(ParseError::new(token.clone(), message))
    }

    fn is_at_end(&self) -> bool {
        return self.peek_token().token_type == TokenType::Eof;
    }

    fn peek_token(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn advance_token(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check_topen_type(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek_token().token_type == token_type;
    }

    fn match_token_type(&mut self, token_type: TokenType) -> bool {
        if self.check_topen_type(token_type) {
            self.advance_token();
            return true;
        }
        return false;
    }

    fn synchronize(&mut self) {
        self.advance_token();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek_token().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For |
                TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
                    return;
                }
                _ => {}
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check_topen_type(token_type) {
            self.advance_token();
        } else {
            let peeked_token = self.peek_token().clone();
            self.error(
                &peeked_token, 
                message);
        }
    }


    fn primary(&mut self) -> Expr {
        println!("Primary {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        if self.match_token_type(TokenType::True) {
            return Expr::Literal { value: LiteralValue::Boolean(true) };
        }
        if self.match_token_type(TokenType::False) {
            return Expr::Literal { value: LiteralValue::Boolean(false) };
        }
        if self.match_token_type(TokenType::Nil) {
            return Expr::Literal { value: LiteralValue::Nil };
        }
        if self.match_token_type(TokenType::NumberLiteral) {
            let value = self.previous().literal.as_ref().unwrap().parse::<f64>().unwrap();
            return Expr::Literal { value: LiteralValue::Number(value) };
        }
        if self.match_token_type(TokenType::StringLiteral) {
            let value = self.previous().literal.as_ref().unwrap().clone();
            return Expr::Literal { value: LiteralValue::String(value) };
        }
        if self.match_token_type(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            self.match_token_type(TokenType::RightParen);
            return Expr::Grouping { expression: Box::new(expr) };
        }
        panic!("Unexpected token: {}", self.peek_token().lexeme);
    }

    fn unary(&mut self) -> Expr {
        // println!("Unary {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        if self.match_token_type(TokenType::Minus) || self.match_token_type(TokenType::Bang) {
            let operator = self.previous().lexeme.clone();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        return self.primary();
    }

    fn factor(&mut self) -> Expr {
        // println!("Factor {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        let mut expr = self.unary();

        while self.match_token_type(TokenType::Star) || self.match_token_type(TokenType::Slash) {
            let operator = self.previous().lexeme.clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn term(&mut self) -> Expr {
        // println!("Term {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        let mut expr = self.factor();

        while self.match_token_type(TokenType::Plus) || self.match_token_type(TokenType::Minus) {
            let operator = self.previous().lexeme.clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        // println!("Comparison {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        let mut expr = self.term();
        // println!("finished calling term, current token: {:?}", self.peek_token());
        while self.match_token_type(TokenType::Greater) || self.match_token_type(TokenType::GreaterEqual) ||
              self.match_token_type(TokenType::Less) || self.match_token_type(TokenType::LessEqual) {
        //    println!("looop starting inside comparison");
            let operator = self.previous().lexeme.clone();
        //    println!("Comparison operator: {}", operator);
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return expr;
    }


    fn equality(&mut self) -> Expr{
        // println!("Equality {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        let mut expr = self.comparison();
        while self.match_token_type(TokenType::EqualEqual) || self.match_token_type(TokenType::BangEqual) {
            let operator_lexeme = self.previous().lexeme.clone();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator_lexeme,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }


    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expression();
        println!("Parsed expression: {:?}", expr);
        if !self.is_at_end() {
            return Err(ParseError::new(self.peek_token().clone(), "Unexpected token after expression"));
        }
        return Ok(expr);
    }

}