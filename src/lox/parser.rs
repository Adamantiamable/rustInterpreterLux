use crate::tool::generate_ast::{Ast, Expr, LiteralValue, Stmt};
use crate::lox::token::{Token, TokenType};
use crate::lox::error_manager::{self, ErrorManager};
use std::rc::Rc;
use std::cell::RefCell;

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
pub struct Parser 
{
    tokens: Vec<Token>,
    current: usize,
    error_manager: Rc<RefCell<ErrorManager>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, error_manager: Rc<RefCell<ErrorManager>>) -> Self {
        Parser {tokens, current:0, error_manager}
    }

    fn error(& mut self, token: &Token, message: &str)-> Result<(), ParseError> {
        self.error_manager.borrow_mut().report(token.line, message, Some(&token.lexeme));
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

    fn check_token_type(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek_token().token_type == token_type;
    }

    fn match_token_type(&mut self, token_type: TokenType) -> bool {
        if self.check_token_type(token_type) {
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
        if self.check_token_type(token_type) {
            self.advance_token();
        } else {
            let peeked_token = self.peek_token().clone();
            self.error(
                &peeked_token, 
                message);
        }
    }


    fn primary(&mut self) -> Expr {
        // println!("Primary {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
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
        //    println!("Number literal: {}", self.previous().literal.as_ref().unwrap());
            let value = self.previous().literal.as_ref().unwrap().parse::<f64>().unwrap();
            return Expr::Literal { value: LiteralValue::Number(value) };
        }
        if self.match_token_type(TokenType::StringLiteral) {
        //    println!("String literal: {}", self.previous().literal.as_ref().unwrap());
            let value = self.previous().literal.as_ref().unwrap().clone();
            return Expr::Literal { value: LiteralValue::String(value) };
        }
        if self.match_token_type(TokenType::LeftParen) {
        //    println!("Left parenthesis found: {:?}", self.previous().lexeme);
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping { expression: Box::new(expr) };
        }
        if self.match_token_type(TokenType::Identifier) {
        //    println!("Identifier has been well identified: {}", self.previous().lexeme);
            return Expr::Variable { name: self.previous().lexeme.clone() };
        
        
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
        //println!("Term {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
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
        //println!("Equality {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
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
        // IF ASSIGNMENT
        if self.peek_token().token_type == TokenType::Identifier && self.tokens.get(self.current + 1).map_or(false, |t| t.token_type == TokenType::Equal) {
            //println!("we are inside assignment");
            self.advance_token(); // Move past the identifier token
            let name = self.previous().lexeme.clone();
            self.consume(TokenType::Equal, "Expect '=' after variable name.");
            let value = self.expression();
            return Expr::Assignment {
                name,
                value: Box::new(value),
            };
        }
        // if not in an assignment, check for logical OR
        self.logical_or()
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        return Stmt::Print(value);
    }

    fn block_statement(&mut self) -> Stmt {
        let mut statements = Vec::new();
        while !self.is_at_end() && !self.check_token_type(TokenType::RightBrace) {
            statements.push(self.statement());
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        return Stmt::Block(statements);
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition.");
        self.consume(TokenType::LeftBrace, "Expect '{' before then body.");

        // Collect all statements in the then branch
        let mut then_statements = Vec::new();
        while !self.check_token_type(TokenType::RightBrace) && !self.is_at_end() {
            then_statements.push(self.statement());
        }
        self.consume(TokenType::RightBrace, "Expect '}' after then body.");

        // Create a sequence statement for then branch
        let then_branch = Box::new(Stmt::Sequence(then_statements));

        // Handle else branch similarly if it exists
        let else_branch = if self.match_token_type(TokenType::Else) {
            self.consume(TokenType::LeftBrace, "Expect '{' before else body.");
            let mut else_statements = Vec::new();
            while !self.check_token_type(TokenType::RightBrace) && !self.is_at_end() {
                else_statements.push(self.statement());
            }
            self.consume(TokenType::RightBrace, "Expect '}' after else body.");
            Some(Box::new(Stmt::Sequence(else_statements)))
        } else {
            None
        };

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }
    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after while condition.");
        self.consume(TokenType::LeftBrace, "Expect '{' before while body.");
        // Collect all statements in the while body
        let mut statements = Vec::new();
        while !self.check_token_type(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.statement());
        }
        self.consume(TokenType::RightBrace, "Expect '}' after while body.");

        let body = Box::new(Stmt::Sequence(statements));

        return Stmt::While {
            condition,
            body,
        }; 
    }


    fn for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");
        // Initializer 
        let initializer = if self.match_token_type(TokenType::Semicolon) {
            None
        } else if self.match_token_type(TokenType::Var) {
            Some(self.var_declaration())
        } else {
            Some(self.expression_statement())
        };
        // Condition
        let condition = if self.check_token_type(TokenType::Semicolon) {
            Expr::Literal { value: LiteralValue::Boolean(true) }
        } else {
            self.expression()
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");
        // Increment
        let increment = if self.check_token_type(TokenType::RightParen) {
            None
        } else {
            Some(self.expression())
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.");
        // Body 
        let mut body = self.statement();
        if let Some(increment_expr) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(increment_expr)]);
        }
        body = Stmt::While {
            condition,
            body: Box::new(body),
        };
        if let Some(init_stmt) = initializer {
            body = Stmt::Block(vec![init_stmt, body]);
        }
        return body;

    }

    fn return_statement(&mut self) -> Stmt {
        let keyword = self.previous().lexeme.clone();
        let value = if !self.is_at_end() && !self.check_token_type(TokenType::Semicolon) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.");
        return Stmt::Return { keyword, value };

    }
    
    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        if self.is_at_end() {
            return Stmt::Expression(expr);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        return Stmt::Expression(expr);
    }

    fn statement(&mut self) -> Stmt {
        if self.match_token_type(TokenType::Print) {
            return self.print_statement();
        }
        if self.match_token_type(TokenType::LeftBrace) {
            return self.block_statement();
        }
        if self.match_token_type(TokenType::If) {
            return self.if_statement();
        }
        if self.match_token_type(TokenType::While) {
            return self.while_statement();
        }
        if self.match_token_type(TokenType::For) {
            return self.for_statement();
        }
        if self.match_token_type(TokenType::Return) {
            return self.return_statement();
        }
        if self.match_token_type(TokenType::Var) {
            return self.var_declaration();
        }
        // Other statement to be added here
        return self.expression_statement();

    }

    fn var_declaration(&mut self) -> Stmt {
        //println!("We are inside var_declaration");
        let name = self.peek_token().lexeme.clone(); // Assuming the variable name is the previous token
        // println!("Variable name: {}", name);
        self.advance_token(); // Move past the variable name token
        let initializer = if self.match_token_type(TokenType::Equal) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.");
        return Stmt::Var { name, initializer }
    }

    fn declaration(&mut self) -> Stmt {
        // println!("Declaration {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        if self.match_token_type(TokenType::Var) {
            return self.var_declaration()
        }
        return self.statement();
        //To add : catch parse errors in which case synchronize
        }

    fn logical_and(&mut self) -> Expr {
        // println!("Logical AND {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        let mut expr = self.equality();
        while self.match_token_type(TokenType::And) {
            let operator = self.previous().lexeme.clone();
            let right = self.equality();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn logical_or(&mut self) -> Expr {
        // println!("Logical OR {:?}, {}", self.peek_token().token_type, self.peek_token().lexeme);
        let mut expr = self.logical_and();
        while self.match_token_type(TokenType::Or) {
            let operator = self.previous().lexeme.clone();
            let right = self.logical_and();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        return statements;
    
    }
   

}