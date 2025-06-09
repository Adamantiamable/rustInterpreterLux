
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
                    "!=" => Ok(LiteralValue::Boolean(&left_value != &right_value)),
                    ">" => match (&left_value, &right_value) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Boolean(l > r)),
                        _ => Err(Error::Runtime("Operands must be numbers".into())),
                    },
                    "<" => match (&left_value, &right_value) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Boolean(l < r)),
                        _ => Err(Error::Runtime("Operands must be numbers".into())),
                    },
                    ">=" => match (&left_value, &right_value) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Boolean(l >= r)),
                        _ => Err(Error::Runtime("Operands must be numbers".into())),
                    },
                    "<=" => match (&left_value, &right_value) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Boolean(l <= r)),
                        _ => Err(Error::Runtime("Operands must be numbers".into())),
                    },
                    "&&" => match (&left_value, &right_value) {
                        (LiteralValue::Boolean(l), LiteralValue::Boolean(r)) => Ok(LiteralValue::Boolean(*l && *r)),
                        _ => Err(Error::Runtime("Operands must be booleans".into())),
                    },
                    "||" => match (&left_value, &right_value) {
                        (LiteralValue::Boolean(l), LiteralValue::Boolean(r)) => Ok(LiteralValue::Boolean(*l || *r)),
                        _ => Err(Error::Runtime("Operands must be booleans".into())),
                    },
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
                //println!("Assigning value to variable: {}", name);
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


    fn interpret_single_statement(&mut self, statement:Stmt) -> Result<LiteralValue, Error> {
            match statement {
                Stmt::Expression(expr) => {
                    self.evaluate(&expr)?;
                },
                Stmt::Print(expr) => {
                    let value = self.evaluate(&expr)?;
                    println!("{:?}", value);
                },
                Stmt::If {condition, then_branch, else_branch} => {
                    println!("Evaluating if statement with condition: {:?}", condition);
                    let condition_value = self.evaluate(&condition)?;
                    if let LiteralValue::Boolean(true) = condition_value {
                        self.interpret_single_statement(*then_branch)?;
                    } else if let Some(else_branch) = else_branch {
                        self.interpret_single_statement(*else_branch)?;
                    }
                },
                Stmt::Var { name, initializer } => {
                    self.execute_var_declaration(&name, initializer.as_ref())?;
                },
                Stmt::Block(statements) => {
                    self.execute_bock(statements)?;
                },
                _ => return Err(self.error_manager.borrow_mut().report_runtime_error(
                    &format!("Unsupported statement type: {:?}", statement))),
        }
        Ok(LiteralValue::Nil) // Return nil if no value is produced
    }

    fn execute_bock(&mut self, statements_list: Vec<Stmt>) -> Result<(), Error> {
        // Save the current environment
        let previous_env = Rc::new(RefCell::new(self.environment.clone()));
    
        // Create a new environment enclosed by the previous one
        self.environment = Environment::new(self.error_manager.clone());
        self.environment.enclosing = Some(previous_env.clone());
    
        for statement in statements_list {
            //println!("Currently working on statement: {:?}", statement);
            if let Err(e) = self.interpret_single_statement(statement) {
                // Optional: print debug info
                println!("Runtime error: {:?}", e);
    
                // Restore the previous environment
                let previous = self.environment.enclosing.as_ref().unwrap().borrow().clone();
                self.environment = previous;
    
                // Return the error
                return Err(e);
            }
        }
    
        // Restore previous environment after block
        self.environment = previous_env.as_ref().borrow().clone();
        Ok(())
    }
    
    
    pub fn interpret(&mut self, statements_list: Vec<Stmt>) -> Result<LiteralValue, Error> {
        for statement in statements_list {
            match statement {
                Stmt::Expression(expr) => {
                    self.evaluate(&expr)?;
                },
                Stmt::If {condition, then_branch, else_branch} => {
                    //println!("Evaluating if statement with condition WITHIN INTERPRET FUNCTION: {:?}", condition);
                    let condition_value = match self.evaluate(&condition) {
                        Ok(value) => {
                            println!("Evaluation successful: {:?}", value);
                            value
                        },
                        Err(e) => {
                            println!("Evaluation failed with error: {:?}", e);
                            return Err(e);
                        }
                    };
                    //println!("Condition value: {:?}", condition_value);
                    if let LiteralValue::Boolean(true) = condition_value {
                        self.interpret_single_statement(*then_branch)?;
                    } else if let Some(else_branch) = else_branch {
                        self.interpret_single_statement(*else_branch)?;
                    }
                },
                Stmt::Print(expr) => {
                    let value = self.evaluate(&expr)?;
                    println!("{:?}", value);
                },
                Stmt::Var { name, initializer } => {
                    self.execute_var_declaration(&name, initializer.as_ref())?;
                },
                Stmt::Block(statements) => {
                    self.execute_bock(statements)?;
                },
                _ => return Err(self.error_manager.borrow_mut().report_runtime_error(
                    &format!("Unsupported statement type: {:?}", statement))),
            }
        }
        Ok(LiteralValue::Nil) // Return nil if no value is produced
    }
}