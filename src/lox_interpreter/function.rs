use core::fmt;
use std::{cell::RefCell, rc::Rc};

use super::{
    ast_tools::Stmt,
    environment::Environment,
    error::LoxError,
    interpreter::{Interpreter, Object},
    token::Token,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}

// TODO: Implement the call and arity methods for the Function struct.
impl Function {
    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &Vec<Object>,
    ) -> Result<Object, LoxError> {
        let env = Rc::new(RefCell::new(Environment::create_enclosing_for_env(
            &self.closure,
        )));

        for (param, arg) in self.params.iter().zip(args.iter()) {
            // NOTE: Borrow checks won't let me do something like let env = env.borrow_mut and then
            // move out so this is what I do.
            env.borrow_mut().define(param.lexeme.clone(), arg.clone())?;
        }

        // println!("Function body: {:?}", self.body);
        match interpreter.execute_block(&self.body, env) {
            Err(LoxError::Return { value }) => {
                // println!("Returning value: {:?}", value);
                return Ok(value);
            }
            Err(other) => Err(other),
            Ok(..) => Ok(Object::NONE),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function: {}", self.name)
    }
}
