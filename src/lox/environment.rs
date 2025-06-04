
use crate::lox::error_manager::ErrorManager;
use crate::tool::generate_ast::LiteralValue;
use std::rc::Rc;
use std::cell::RefCell;
use crate::lox::token::Token;

pub struct Environment {
    values: std::collections::HashMap<String, LiteralValue>,
    error_manager: Rc<RefCell<ErrorManager>>,
}

impl Environment {
    pub fn new(error_manager: Rc<RefCell<ErrorManager>>) -> Self {
        Environment {
            values: std::collections::HashMap::new(),
            error_manager,
        }
        
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &str) -> Option<&LiteralValue> {
        if !self.values.contains_key(name) {
            self.error_manager.borrow_mut().report_runtime_error(
                &format!("Undefined variable '{}'.", name)
            );
            return None;
        }
        self.values.get(name)
    }

    
}