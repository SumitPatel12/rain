// NOTE: I really don't know where the calss generation is taknig me, will skip that for now and
// see where it takes me.

use super::token::Token;

#[allow(unused_imports)]
use crate::lox_interpreter::token::{Literal, TokenType};

pub trait Visitor<T> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&self, expression: &Expr) -> T;
    fn visit_literal_expr(&self, value: String) -> T;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> T;
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
        value: String,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value.to_string()),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}

// NOTE: This was harder than it looks.
pub struct ASTPrinter;

impl Visitor<String> for ASTPrinter {
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        // NOTE: Yup, you need to clone this, same goes for the one below.
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }

    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_literal_expr(&self, value: String) -> String {
        // TODO: Check if the None case should be handled differently as they have done in the book.
        value
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.parenthesize("group".to_string(), vec![expression])
    }
}

impl ASTPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    // NOTE: format does return a string, but I couldn't figure out a good way to use that.
    pub fn parenthesize(&self, name: String, expressions: Vec<&Expr>) -> String {
        let mut parenthesized_string = String::new();
        parenthesized_string.push('(');
        parenthesized_string.push_str(&name);

        for expression in expressions {
            parenthesized_string.push(' ');
            parenthesized_string.push_str(&expression.accept(self));
        }
        parenthesized_string.push(')');
        parenthesized_string
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
                value: "123".to_string(),
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
                value: "45.67".to_string(),
            }),
        }),
    };
    let printer = ASTPrinter;
    assert_eq!(printer.print(expression), "(* (- 123) (group 45.67))");
}
