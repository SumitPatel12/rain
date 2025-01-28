// NOTE: I really don't know where the calss generation is taknig me, will skip that for now and
// see where it takes me.
use super::token::Token;

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

// TODO: Implement AST printer.

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Don't forget to add tests for this one, scanner was an exception.
}
