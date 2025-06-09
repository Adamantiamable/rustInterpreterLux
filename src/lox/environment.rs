
use crate::lox::error_manager::ErrorManager;
use crate::tool::generate_ast::LiteralValue;
use std::rc::Rc;
use std::cell::RefCell;
use crate::lox::token::Token;

#[derive(Debug, Clone)]
pub struct Environment {
    values: std::collections::HashMap<String, LiteralValue>,
    error_manager: Rc<RefCell<ErrorManager>>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(error_manager: Rc<RefCell<ErrorManager>>) -> Self {
        Environment {
            values: std::collections::HashMap::new(),
            error_manager,
            enclosing: None,
        }
        
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &str) -> Option<LiteralValue> {
        if self.values.contains_key(name) {
            return self.values.get(name).cloned();
        } else if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().get(name);
        }  
        self.error_manager.borrow_mut().report_runtime_error(
        &format!("Undefined variable '{}'.", name));
        None
    }


    
}