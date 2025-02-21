use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{error::LoxError, interpreter::Object, token::Token};

pub struct Environment {
    // NOTE: Without Box or Rc it comlains about struct having infinite size.
    // Going with Rc cause it seemed like the right call to have it under a mutex, i.e. all blocks
    // share the same parent kind of thing.
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
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
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        let key = &name.lexeme;
        //println!("Trying to get value from environment for key: {}", key);
        if let Some(value) = self.values.get(key) {
            //println!("GOT VALUE.");
            Ok(value.clone())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                //println!("Trying outer scope.");
                enclosing.borrow().get(name)
            } else {
                //println!("In Error get.");
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
