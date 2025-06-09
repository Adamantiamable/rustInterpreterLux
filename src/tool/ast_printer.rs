use crate::lox::token::{Token, TokenType};
use crate::tool::generate_ast::{Expr, LiteralValue};
pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                format!("({} {} {})", operator, self.print(left), self.print(right))
            }
            Expr::Grouping { expression } => {
                format!("(group {})", self.print(expression))
            }

            Expr::Literal { value } => {
                match value {
                    LiteralValue::Number(n) => n.to_string(),
                    LiteralValue::String(s) => format!("\"{}\"", s),
                    LiteralValue::Boolean(b) => b.to_string(),
                    LiteralValue::Nil => "nil".to_string(),
                }
            }
            Expr::Unary { operator, right } => {
                format!("({} {})", operator, self.print(right))
            }
            Expr::Variable { name } => {
                name.clone()
            }
            Expr::Assignment { name, value } => {
                format!("({} = {})", name, self.print(value))
            }
            Expr::Logical { left, operator, right } => {
                format!("({} {} {})", operator, self.print(left), self.print(right))
            }
        }
    }
    pub fn main() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: "-".to_string(),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(123.0),
            }),
        }),
        operator: "*".to_string(),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        }),
    };

    let printer = AstPrinter::new();
    let output = printer.print(&expression);
    println!("{}", output);
}
}

pub fn main() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: "-".to_string(),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(123.0),
            }),
        }),
        operator: "*".to_string(),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        }),
    };

    let printer = AstPrinter::new();
    let output = printer.print(&expression);
    println!("{}", output);
}