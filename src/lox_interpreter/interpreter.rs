use crate::lox_interpreter::error::LoxError;
use std::{cell::RefCell, fmt, rc::Rc};

use super::{
    ast_tools::{expr, stmt, Expr, Stmt},
    environment::Environment,
    function::Function,
    token::{Literal, Token, TokenType},
};

// TODO: Add a callable class
#[derive(Clone, Debug)]
pub enum Object {
    Boolean(bool),
    NONE,
    Number(f64),
    String(String),
    Callable(Function),
}

impl Object {
    fn equals(&self, value: &Object) -> bool {
        match (self, value) {
            (Object::Boolean(l), Object::Boolean(r)) => l == r,
            (Object::NONE, Object::NONE) => true,
            (Object::NONE, _) => false,
            (_, Object::NONE) => false,
            (Object::Number(l), Object::Number(r)) => l == r,
            (Object::String(l), Object::String(r)) => l.eq(r),
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::NONE => write!(f, "null"),
            Object::Callable(fun) => write!(f, "Object callable function: {}", fun.name.lexeme),
        }
    }
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    #[allow(dead_code)]
    fn stringify(&self, object: Object) -> String {
        match object {
            Object::NONE => "None".to_string(),
            Object::Number(n) => n.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::String(s) => s,
            Object::Callable(f) => f.to_string(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in statements {
            self.execute(&stmt)?
        }
        Ok(())
    }

    fn evaluate(&mut self, expression: &Expr) -> Result<Object, LoxError> {
        expression.accept(self)
    }

    fn is_truthly(&self, right: &Object) -> bool {
        match right {
            Object::NONE => false,
            Object::Boolean(bool) => *bool,
            _ => true,
        }
    }

    fn number_operand_error<T>(&self, operator: &Token, message: String) -> Result<T, LoxError> {
        let error_message = if message.is_empty() {
            "Operand must be a number.".to_string()
        } else {
            message
        };

        Err(LoxError::Runtime {
            token: operator.clone(),
            message: error_message,
        })
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)?;
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), LoxError> {
        let previous = self.environment.clone();
        // NOTE: Turns out closures can be used like this as well, this was kindof a round about
        // way of getting over the try catch thing in java.
        let execue_statements = || -> Result<(), LoxError> {
            self.environment = environment;
            for statement in statements {
                self.execute(statement)?
            }

            Ok(())
        };

        let result = execue_statements();
        self.environment = previous;

        result
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &super::token::Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;

        match operator.token_type {
            TokenType::BANG_EQUAL => Ok(Object::Boolean(!l.equals(&r))),
            TokenType::EQUAL_EQUAL => Ok(Object::Boolean(l.equals(&r))),
            TokenType::LESS => match (l, r) {
                (Object::Number(l_number), Object::Number(r_number)) => {
                    Ok(Object::Boolean(l_number < r_number))
                }
                (Object::String(l_string), Object::String(r_string)) => {
                    Ok(Object::Boolean(l_string < r_string))
                }
                _ => self.number_operand_error(operator, String::new()),
            },
            TokenType::LESS_EQUAL => match (l, r) {
                (Object::Number(l_number), Object::Number(r_number)) => {
                    Ok(Object::Boolean(l_number <= r_number))
                }
                (Object::String(l_string), Object::String(r_string)) => {
                    Ok(Object::Boolean(l_string <= r_string))
                }
                _ => self.number_operand_error(operator, String::new()),
            },
            TokenType::GREATER => match (l, r) {
                (Object::Number(l_number), Object::Number(r_number)) => {
                    Ok(Object::Boolean(l_number > r_number))
                }
                (Object::String(l_string), Object::String(r_string)) => {
                    Ok(Object::Boolean(l_string > r_string))
                }
                _ => self.number_operand_error(operator, String::new()),
            },
            TokenType::GREATER_EQUAL => match (l, r) {
                (Object::Number(l_number), Object::Number(r_number)) => {
                    Ok(Object::Boolean(l_number >= r_number))
                }
                (Object::String(l_string), Object::String(r_string)) => {
                    Ok(Object::Boolean(l_string >= r_string))
                }
                _ => self.number_operand_error(operator, String::new()),
            },
            TokenType::PLUS => match (l, r) {
                (Object::Number(l_number), Object::Number(r_number)) => {
                    Ok(Object::Number(l_number + r_number))
                }
                (Object::String(l_string), Object::String(r_string)) => {
                    Ok(Object::String(l_string + &r_string))
                }
                _ => self.number_operand_error(operator, String::new()),
            },
            TokenType::MINUS => match (l, r) {
                (Object::Number(l_number), Object::Number(r_number)) => {
                    Ok(Object::Number(l_number - r_number))
                }
                _ => self.number_operand_error(operator, String::new()),
            },
            TokenType::STAR => match (l, r) {
                (Object::Number(l_number), Object::Number(r_number)) => {
                    Ok(Object::Number(l_number * r_number))
                }
                _ => self.number_operand_error(operator, String::new()),
            },
            TokenType::SLASH => match (l, r) {
                (Object::Number(l_number), Object::Number(r_number)) => {
                    if r_number == 0.0 {
                        return self.number_operand_error(operator, "Divide by zero.".to_string());
                    }
                    return Ok(Object::Number(l_number / r_number));
                }
                _ => self.number_operand_error(operator, String::new()),
            },
            _ => self.number_operand_error(operator, String::new()),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, LoxError> {
        self.evaluate(expression)
    }

    fn visit_literal_expr(&mut self, value: &Literal) -> Result<Object, LoxError> {
        match value {
            Literal::String(str) => Ok(Object::String(str.clone())),
            Literal::Float(f) => Ok(Object::Number(*f)),
            Literal::None => Ok(Object::NONE),
            Literal::Boolean(boolean) => Ok(Object::Boolean(*boolean)),
        }
    }

    fn visit_unary_expr(
        &mut self,
        operator: &super::token::Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::MINUS => match right {
                Object::Number(num) => Ok(Object::Number(-num)),
                _ => self.number_operand_error(operator, String::new()),
            },
            TokenType::BANG => Ok(Object::Boolean(self.is_truthly(&right))),
            _ => self.number_operand_error(operator, String::new()),
        }
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, LoxError> {
        self.environment.borrow_mut().get(name)
    }

    fn visit_assignment_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, LoxError> {
        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
        let left_value = self.evaluate(left)?;

        if operator.token_type == TokenType::OR {
            if self.is_truthly(&left_value) {
                return Ok(left_value);
            }
        } else {
            if !self.is_truthly(&left_value) {
                return Ok(left_value);
            }
        }

        self.evaluate(right)
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<Object, LoxError> {
        // TODO: Check if the callee is in fact callable and if not throw an error.
        let callee_evaluated = self.evaluate(callee)?;

        // TODO: Fk I'll have to check itreaters in more detail, don't know shit about them. That
        // and closures I think.
        let arguments_evaluated: Result<Vec<Object>, LoxError> = arguments
            .into_iter()
            .map(|expr| self.evaluate(expr))
            .collect();
        let args = arguments_evaluated?;

        if let Object::Callable(function) = callee_evaluated {
            if args.len() != function.arity() {
                Err(LoxError::Runtime {
                    token: paren.clone(),
                    message: format!(
                        "Expected {} arguments, but got {}.",
                        function.arity(),
                        args.len()
                    ),
                })
            } else {
                function.call(self, &args)
            }
        } else {
            Err(LoxError::Runtime {
                token: paren.clone(),
                message: "Can only call functions.".to_string(),
            })
        }
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_block_stmt(
        &mut self,
        statements: &Vec<super::ast_tools::Stmt>,
    ) -> Result<(), LoxError> {
        self.execute_block(
            statements,
            Rc::new(RefCell::new(Environment::create_enclosing_for_env(
                &self.environment,
            ))),
        )?;
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), LoxError> {
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), LoxError> {
        let value = self.evaluate(expression)?;
        println!("Visiting Print Statement: {}", self.stringify(value));
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<(), LoxError> {
        match initializer {
            Some(init) => {
                let value = self.evaluate(init)?;
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value)?;
                return Ok(());
            }
            None => {
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Object::NONE)?;
                return Ok(());
            }
        }
    }

    fn visit_if_statement(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Result<(), LoxError> {
        let value = self.evaluate(condition)?;
        if self.is_truthly(&value) {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_statement(
        &mut self,
        condition: &Expr,
        body: &Box<Stmt>,
    ) -> Result<(), LoxError> {
        // NOTE: Once again if you're thinking of using something like
        // `self.is_trutly(&self.evalutate(condition)?)`
        // Think again. We're trying to borrow mutalby return it's referece inside of a function
        // that is immutbly borrowing the class. So, it's gonna complain.
        // You see assigning the value ends the borrow as we surrender the ownership to the local
        // variable, but directly calling it withing the function does not.
        let mut condition_value = self.evaluate(condition)?;
        self.environment.borrow_mut().is_enclosed_in_loop = true;
        while self.is_truthly(&condition_value) {
            let body_execution_result = self.execute(body);
            if let Err(LoxError::BreakStmtError) = body_execution_result {
                break;
            } else if let Err(LoxError::Return { value }) = body_execution_result {
                return Err(LoxError::Return { value });
            } else if let Err(LoxError::ContinueStmtError) = body_execution_result {
                condition_value = self.evaluate(condition)?;
                continue;
            } else {
                condition_value = self.evaluate(condition)?;
            }
        }

        // This most certainly would have performance overheads, but I can't think of anything
        // more graceful.
        self.environment.borrow_mut().is_enclosed_in_loop = false;

        Ok(())
    }

    fn visit_break_stmt(&mut self) -> Result<(), LoxError> {
        if self.environment.borrow().is_enclosed_in_loop {
            return Err(LoxError::BreakStmtError);
        }
        return Ok(());
    }

    fn visit_continue_stmt(&mut self) -> Result<(), LoxError> {
        if self.environment.borrow().is_enclosed_in_loop {
            return Err(LoxError::ContinueStmtError);
        }
        return Ok(());
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        paramaters: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), LoxError> {
        let function = Function {
            name: name.clone(),
            params: paramaters.clone(),
            body: body.clone(),
            closure: Rc::clone(&self.environment),
        };

        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Object::Callable(function))?;
        Ok(())
    }

    fn visit_return_stmt(
        &mut self,
        _keyword: &Token,
        value: &Option<Expr>,
    ) -> Result<(), LoxError> {
        let return_value = if let Some(val) = value {
            self.evaluate(val)?
        } else {
            Object::NONE
        };

        //println!("In Return Stmt: {:?}", return_value);
        Err(LoxError::Return {
            value: return_value,
        })
    }
}
