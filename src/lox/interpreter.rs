
use crate::tool::generate_ast::LiteralValue;
use crate::{lox::error_manager::ErrorManager, tool::generate_ast::{Expr, Stmt}, lox::token::TokenType};
use crate::lox::error_manager::{self, Error};
use crate::lox::environment::Environment;
use std::rc::Rc;
use std::cell::RefCell;


pub struct Interpreter {
    error_manager: Rc<RefCell<ErrorManager>>,
    environment: Environment,
    }

impl Interpreter {
    pub fn new(error_manager: Rc<RefCell<ErrorManager>>) -> Self {
        Interpreter { 
            error_manager: error_manager.clone(), 
            environment: Environment::new(error_manager), // or pass the appropriate parent environment if needed
        }
    }

    fn check_number_operand(&mut self, operator: &str, operand: &LiteralValue) -> Result<(), Error> {
        if let LiteralValue::Number(_) = operand {
            Ok(())
        } else {
            Err(self.error_manager.borrow_mut().report_runtime_error(
                &format!("Operand must be a number for operator '{}'", operator)))
            
        }
    }

    pub fn evaluate (&mut self, expression: &Expr) -> Result<LiteralValue, Error> {
        match expression {
            Expr::Literal{value} => {
                Ok(value.clone())
            },
            Expr::Binary { left, operator, right } => {
                let left_value = self.evaluate(left)?;
                self.check_number_operand(operator, &left_value)?;
                let right_value = self.evaluate(right)?;
                self.check_number_operand(operator, &right_value)?;
                match operator.as_str() {
                    "+" => match (&left_value, &right_value) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Number(l + r)),
                        _ => Err(Error::Runtime("Operands must be numbers".into())),
                    },
                    "-" => match (&left_value, &right_value) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Number(l - r)),
                        _ => Err(Error::Runtime("Operands must be numbers".into())),
                    },
                    "*" => match (&left_value, &right_value) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Number(l * r)),
                        _ => Err(Error::Runtime("Operands must be numbers".into())),
                    },
                    "/" => match (&left_value, &right_value) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Number(l / r)),
                        _ => Err(Error::Runtime("Operands must be numbers".into())),
                    },
                    "==" => Ok(LiteralValue::Boolean(&left_value == &right_value)),
                    _ => Err(Error::Runtime("Invalid operator for binary expression".into())),
                }
            },
            Expr::Unary { operator, right } => {
                let right_value = self.evaluate(right)?;
                match operator.as_str() {
                    "-" => match (&right_value) {
                        LiteralValue::Number(r) => Ok(LiteralValue::Number(-r)),
                        _ => Err(Error::Runtime("Right value must be a number".into())),
                    },
                    "!" => match (&right_value) {
                        (LiteralValue::Boolean(r)) => Ok(LiteralValue::Boolean(!r)),
                        _ => Err(Error::Runtime("Right value must be a boolean".into())),
                    },
                    _ => Err(self.error_manager.borrow_mut().report_runtime_error(
                        &format!("Invalid operator '{}' for unary expression", operator))),
            }
            },
            Expr::Grouping { expression } => {
                self.evaluate(expression)
            },
            Expr::Variable { name } => {
                if let Some(value) = self.environment.get(name) {
                    Ok(value.clone())
                } else {
                    Err(self.error_manager.borrow_mut().report_runtime_error(
                        &format!("Undefined variable '{}'.", name)))
                }
            },
            Expr::Assignment { name, value }
            => {
                println!("Assigning value to variable: {}", name);
                let value = self.evaluate(value)?;
                self.environment.define(name.clone(), value.clone());
                Ok(value)
            },
            _ => Err(self.error_manager.borrow_mut().report_runtime_error(
                &format!("Unexpected expression type: {:?}", expression)))
        }
    }
    pub fn execute_var_declaration(&mut self, name: &str, initializer: Option<&Expr>) -> Result<(), Error> {
        let value = if let Some(expr) = initializer {
            self.evaluate(expr)?
        } else {
            LiteralValue::Nil // Default value if no initializer is provided
        };
        self.environment.define(name.to_string(), value);
        Ok(())
    }
    
    pub fn interpret(&mut self, statements_list: Vec<Stmt>) -> Result<LiteralValue, Error> {
        for statement in statements_list {
            match statement {
                Stmt::Expression(expr) => {
                    self.evaluate(&expr)?;
                },
                Stmt::Print(expr) => {
                    let value = self.evaluate(&expr)?;
                    println!("{:?}", value);
                },
                Stmt::Var { name, initializer } => {
                    self.execute_var_declaration(&name, initializer.as_ref())?;
                },
                _ => return Err(self.error_manager.borrow_mut().report_runtime_error(
                    &format!("Unsupported statement type: {:?}", statement))),
            }
        }
        Ok(LiteralValue::Nil) // Return nil if no value is produced
    }
}