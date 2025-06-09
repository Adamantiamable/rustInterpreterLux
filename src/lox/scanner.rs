use crate::lox::token::{Token};
use crate::lox::token_type::TokenType;
use crate::lox::error_manager::ErrorManager;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::rc::Rc;
use std::cell::RefCell;


static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);
    m
});

pub struct Scanner
{
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_manager: Rc<RefCell<ErrorManager>>, 
}


impl Scanner {
    pub fn new(source: String, error_manager: Rc<RefCell<ErrorManager>>) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_manager,
        }
    }

    
    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char
    {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, token_type:TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        //println!("Adding token: {:?} with text: {}", token_type, text);
        self.tokens.push(Token::new(token_type, text.to_string(), literal, self.line));

    }
     
    fn match_char(&mut self, expected:char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'; // Return null character if at end
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0'; // Return null character if at end
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn is_digit(&self, c:char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    fn number(&mut self) {
        let mut has_decimal = false;
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            has_decimal = true;
            self.advance();
            while self.is_digit(self.peek()) {self.advance();}
        }
        self.add_token(TokenType::NumberLiteral, Some(self.source[self.start..self.current].to_string()));

    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line +=1;
            }
            self.advance();
        }
        
        if self.is_at_end() {
            self.error_manager.borrow_mut().report(self.line, "Unterminated string.", None);
            return;
        }
        else {
            self.advance();
            let value = self.source[self.start + 1..self.current-1].to_string();
            self.add_token(TokenType::StringLiteral, Some(value));
        }
    }
    
    fn identifier(&mut self) {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {self.advance();}
        let token_type = KEYWORDS
            .get(&self.source[self.start..self.current])
            .unwrap_or(&TokenType::Identifier);
        self.add_token(token_type.clone(), Some(self.source[self.start..self.current].to_string()));
    }


    fn scan_token(&mut self)
    {
        self.start = self.current;
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            },
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }   
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            },
            '/' => {
                if self.match_char('/') {
                    // Single-line comment 
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                else {
                    self.add_token(TokenType::Slash, None);
                }
                
            },
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            },
            '\n' => {
                self.line += 1;
            },
            '"' => {self.string()},
            _ => {
                if self.is_digit(c) {
                    self.number();
                }
                else if self.is_alpha(c)  {
                self.identifier();
                }
                else {
                self.error_manager.borrow_mut().report(self.line, &format!("Unexpected character: '{}'", c), None);
                }
            }
        }
        
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {


        while !self.is_at_end() {
            let start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        return self.tokens.clone();
    
    }
    
    }