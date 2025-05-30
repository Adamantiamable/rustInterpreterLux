use crate::tool::generate_ast::LiteralValue;
use crate::{lox::error_manager::ErrorManager, tool::generate_ast::Expr, lox::token::TokenType};
use crate::lox::error_manager::Error;

use super::error_manager;


pub struct Interpreter <'a> {
    error_manager: &'a mut ErrorManager,
    }

impl <'a> Interpreter <'a> {
    pub fn new(error_manager: &'a mut ErrorManager) -> Self {
        Interpreter { error_manager }
    }

    fn check_number_operand(&mut self, operator: &str, operand: &LiteralValue) -> Result<(), Error> {
        if let LiteralValue::Number(_) = operand {
            Ok(())
        } else {
            Err(self.error_manager.report_runtime_error(
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
                    _ => Err(self.error_manager.report_runtime_error(
                        &format!("Invalid operator '{}' for unary expression", operator))),
            }
            },
            Expr::Grouping { expression } => {
                self.evaluate(expression)
            },
            _ => Err(self.error_manager.report_runtime_error(
                &format!("Unexpected expression type: {:?}", expression)))
        }
    }
    
}