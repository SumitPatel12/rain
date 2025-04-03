use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{error::LoxError, interpreter::Object, token::Token};

#[derive(Debug)]
pub struct Environment {
    // NOTE: Without Box or Rc it complains about struct having infinite size.
    // Going with Rc cause it seemed like the right call to have it under a mutex, i.e. all blocks
    // share the same parent kind of thing.
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
    pub is_enclosed_in_loop: bool,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
            is_enclosed_in_loop: false,
        }
    }

    pub fn define(&mut self, name: String, value: Object) -> Result<(), LoxError> {
        self.values.insert(name, value);
        Ok(())
    }

    pub fn create_enclosing_for_env(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
            is_enclosed_in_loop: enclosing.borrow().is_enclosed_in_loop,
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        let key = &name.lexeme;
        if let Some(value) = self.values.get(key) {
            Ok(value.clone())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow().get(name)
            } else {
                Err(LoxError::Runtime {
                    token: name.clone(),
                    message: format!("Undefined variable: {}", key),
                })
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxError> {
        let key = &name.lexeme;
        if self.values.contains_key(key) {
            self.values.insert(key.clone(), value);
            Ok(())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow_mut().assign(name, value)
            } else {
                Err(LoxError::Runtime {
                    token: name.clone(),
                    message: format!("Undefined variable: {}", key),
                })
            }
        }
    }
}
