

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum Expr{
    Binary{ // 1+2
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Grouping{ // (xxx)
        expression: Box<Expr>,
    },
    Literal{ // 1, "hello", true, false, nil
        value: LiteralValue,
    },
    Unary{ // -1, !true
        operator: String,
        right: Box<Expr>,
    },
    Variable{ // x
        name: String,
    },
    Assignment{ // x = 1
        name: String,
        value: Box<Expr>,
    },
}
#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: String,
        parameters: Vec<String>,
        body: Vec<Stmt>,
    },
    Return {
        keyword: String,
        value: Option<Expr>,
    },
    Class {
        name: String,
        superclass: Option<String>,
        methods: Vec<Stmt>,
    },
    Break,
    Continue,
    Empty,
    Error(String), // For error handling
}

pub struct Ast {
    
    pub ast: String,
    pub statements_list: Vec<Stmt>,
}


impl Ast {
    pub fn new(statements_list: Vec<Stmt>) -> Self {
        Ast {
            ast: String::new(),
            statements_list,
        }
    }

    pub fn generate_ast(&mut self) {
        for stmt in &self.statements_list {
            println!()
            }
        }
    }