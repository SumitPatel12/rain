use std::fmt;

// NOTE: I really don't know where the calss generation is taknig me, will skip that for now and
// see where it takes me.
use super::{error::LoxError, token::Token};

#[allow(unused_imports)]
use crate::lox_interpreter::token::{Literal, TokenType};

pub mod expr {
    use super::Expr;
    use crate::lox_interpreter::{
        error::LoxError,
        token::{Literal, Token},
    };

    pub trait Visitor<T> {
        fn visit_binary_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<T, LoxError>;
        fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<T, LoxError>;
        fn visit_literal_expr(&mut self, value: &Literal) -> Result<T, LoxError>;
        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<T, LoxError>;
        fn visit_variable_expr(&mut self, name: &Token) -> Result<T, LoxError>;
        fn visit_assignment_expr(&mut self, name: &Token, value: &Expr) -> Result<T, LoxError>;
        fn visit_logical_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<T, LoxError>;
        fn visit_call_expr(
            &mut self,
            callee: &Expr,
            pren: &Token,
            arguments: &Vec<Expr>,
        ) -> Result<T, LoxError>;
    }
}

// NOTE: I tried using a trait for Expr and it's not a good idea. You run into the exact problem
// you're trying to avoid, implementing something for every "class".
#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn expr::Visitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            // TODO: Add relevant visitor methods.
            Expr::Assign { name, value } => visitor.visit_assignment_expr(name, value),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expr(left, operator, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call_expr(callee, paren, arguments),
        }
    }
}

pub mod stmt {
    use super::{Expr, Stmt};
    use crate::lox_interpreter::{error::LoxError, token::Token};

    pub trait Visitor<T> {
        fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<T, LoxError>;
        fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<T, LoxError>;
        fn visit_print_stmt(&mut self, expression: &Expr) -> Result<T, LoxError>;
        fn visit_var_stmt(
            &mut self,
            name: &Token,
            initializer: &Option<Expr>,
        ) -> Result<T, LoxError>;
        fn visit_if_statement(
            &mut self,
            condition: &Expr,
            then_branch: &Stmt,
            else_branch: &Option<Stmt>,
        ) -> Result<T, LoxError>;
        fn visit_while_statement(
            &mut self,
            condition: &Expr,
            body: &Box<Stmt>,
        ) -> Result<T, LoxError>;
        fn visit_break_stmt(&mut self) -> Result<T, LoxError>;
        fn visit_continue_stmt(&mut self) -> Result<T, LoxError>;
        fn visit_function_stmt(
            &mut self,
            name: &Token,
            paramaters: &Vec<Token>,
            body: &Vec<Stmt>,
        ) -> Result<T, LoxError>;
        fn visit_return_stmt(
            &mut self,
            keyword: &Token,
            value: &Option<Expr>,
        ) -> Result<T, LoxError>;
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Break,
    Continue,
    Block {
        statements: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    },
    Function {
        name: Token,
        paramaters: Vec<Token>,
        body: Vec<Stmt>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    NONE,
}

impl Stmt {
    /// Used to direct the flow to the correct visitor function.
    pub fn accept<T>(&self, visitor: &mut dyn stmt::Visitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Print {
                expression: expresson,
            } => visitor.visit_print_stmt(expresson),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::NONE => unimplemented!(),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_statement(condition, then_branch, else_branch),
            Stmt::While { condition, body } => visitor.visit_while_statement(condition, body),
            Stmt::Break => visitor.visit_break_stmt(),
            Stmt::Continue => visitor.visit_continue_stmt(),
            Stmt::Function {
                name,
                paramaters,
                body,
            } => visitor.visit_function_stmt(name, paramaters, body),
            Stmt::Return { keyword, value } => visitor.visit_return_stmt(keyword, value),
        }
    }
}

// NOTE: This was harder than it looks.
pub struct ASTPrinter;

impl expr::Visitor<String> for ASTPrinter {
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<String, LoxError> {
        // NOTE: Yup, you need to clone this, same goes for the one below.
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, LoxError> {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_literal_expr(&mut self, value: &Literal) -> Result<String, LoxError> {
        // TODO: Check if the None case should be handled differently as they have done in the book.
        Ok(value.to_string())
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<String, LoxError> {
        self.parenthesize("group".to_string(), vec![expression])
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<String, LoxError> {
        Ok(name.lexeme.clone())
    }

    fn visit_assignment_expr(&mut self, name: &Token, value: &Expr) -> Result<String, LoxError> {
        self.parenthesize(name.lexeme.clone(), vec![value])
    }

    // TODO: Do I really want to develop the printer class any further?
    #[allow(unused)]
    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, LoxError> {
        todo!()
    }

    #[allow(unused)]
    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        pren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<String, LoxError> {
        todo!()
    }
}

impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter
    }

    pub fn print(&mut self, expr: Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }

    // NOTE: format does return a string, but I couldn't figure out a good way to use that.
    pub fn parenthesize(
        &mut self,
        name: String,
        expressions: Vec<&Expr>,
    ) -> Result<String, LoxError> {
        let mut parenthesized_string = String::new();
        parenthesized_string.push('(');
        parenthesized_string.push_str(&name);

        for expression in expressions {
            parenthesized_string.push(' ');
            parenthesized_string.push_str(&expression.accept(self)?);
        }
        parenthesized_string.push(')');
        Ok(parenthesized_string)
    }
}

// NOTE: Finally got them to work, don't ask me how much dry running I did on this one.
#[test]
fn test_print_tree() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            // Line and column do not matter for this.
            operator: Token::new(
                TokenType::MINUS,
                "-".to_string(),
                Literal::String("-".to_string()),
                1,
                0,
            ),
            right: Box::new(Expr::Literal {
                value: Literal::Float(123f64),
            }),
        }),
        // Line and column do not matter for this.
        operator: Token::new(
            TokenType::STAR,
            "*".to_string(),
            Literal::String("*".to_string()),
            1,
            0,
        ),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Literal::Float(45.67),
            }),
        }),
    };
    let mut printer = ASTPrinter;
    assert_eq!(
        printer.print(expression).unwrap(),
        "(* (- 123) (group 45.67))"
    );
}
