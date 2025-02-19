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
        fn visit_assignment_expression(
            &mut self,
            name: &Token,
            value: &Expr,
        ) -> Result<T, LoxError>;
    }
}

// NOTE: I tried using a trait for Expr and it's not a good idea. You run into the exact problem
// you're trying to avoid, implementing something for every "class".
pub enum Expr {
    Binary {
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
            Expr::Assign { name, value } => visitor.visit_assignment_expression(name, value),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
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
    }
}

pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
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
    NONE,
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn stmt::Visitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Print {
                expression: expresson,
            } => visitor.visit_print_stmt(expresson),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::NONE => todo!(),
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

    fn visit_assignment_expression(
        &mut self,
        name: &Token,
        value: &Expr,
    ) -> Result<String, LoxError> {
        self.parenthesize(name.lexeme.clone(), vec![value])
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
