#![allow(unused)]

// builder.rs - Builder pattern implementation

use crate::token::{Operator, Token};

#[derive(Debug, Clone)]
pub struct Expression {
    tokens: Vec<Token>,
}

#[derive(Default)]
pub struct ExpressionBuilder {
    tokens: Vec<Token>,
    paren_count: i32, // Track parentheses balance
}

// Consuming Builder (Owned self) //

impl ExpressionBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    // Add a number to the expression
    pub fn number(mut self, value: f64) -> Self {
        self.tokens.push(Token::number(value));
        self
    }

    // Add an operator
    pub fn operator(mut self, op: Operator) -> Self {
        self.tokens.push(Token::operator(op));
        self
    }

    // Add a variable
    pub fn variable(mut self, name: impl Into<String>) -> Self {
        self.tokens.push(Token::variable(name));
        self
    }

    // Open a parenthesis group
    pub fn open_paren(mut self) -> Self {
        self.tokens.push(Token::OpenParen);
        self.paren_count += 1;
        self
    }

    // Close a parenthesis group
    pub fn close_paren(mut self) -> Result<Self, String> {
        if self.paren_count <= 0 {
            return Err("Unmatched closing parenthesis".to_string());
        }
        self.tokens.push(Token::CloseParen);
        self.paren_count -= 1;
        Ok(self)
    }

    // Build the final expression
    pub fn build(self) -> Result<Expression, String> {
        if self.paren_count != 0 {
            return Err("Unmatched parentheses".to_string());
        }

        if self.tokens.is_empty() {
            return Err("Empty expression".to_string());
        }

        // Validate the expression structure
        self.validate_expression()?;

        Ok(Expression {
            tokens: self.tokens,
        })
    }

    fn validate_expression(&self) -> Result<(), String> {
        // This is a simplistic validation - in a real calculator
        // this would be much more thorough

        if self.tokens.is_empty() {
            return Err("Expression cannot be empty".to_string());
        }

        // Make sure we don't have consecutive operators
        let mut prev_is_op = false;

        for token in &self.tokens {
            match token {
                Token::Operator(_) => {
                    if prev_is_op {
                        return Err("Consecutive operators not allowed".to_string());
                    }
                    prev_is_op = true;
                }
                _ => prev_is_op = false,
            }
        }

        Ok(())
    }
}

// Additional builder methods for common expression patterns
impl ExpressionBuilder {
    // Binary operation (like "2 + 3")
    pub fn binary_op(self, left: f64, op: Operator, right: f64) -> Self {
        self.number(left).operator(op).number(right)
    }

    // Function application (like "sin(x)")
    pub fn function_call(self, func: crate::token::Function, arg: impl Into<String>) -> Self {
        self.function(func)
            .open_paren()
            .variable(arg)
            .close_paren()
            .unwrap() // Safe because we're matching parens
    }

    fn function(mut self, func: crate::token::Function) -> Self {
        self.tokens.push(Token::function(func));
        self
    }
}

// Template methods for common expressions
impl Expression {
    // 1.0 * x ^ 2.0 + 0.0 * x + 0.0 = x ^ 2.0
    pub fn quadratic() -> ExpressionBuilder {
        ExpressionBuilder::new()
            .number(1.0) // Default a coefficient
            .operator(Operator::Multiply)
            .variable("x")
            .operator(Operator::Power)
            .number(2.0)
            .operator(Operator::Add)
            .number(0.0) // Default b coefficient
            .operator(Operator::Multiply)
            .variable("x")
            .operator(Operator::Add)
            .number(0.0) // Default c coefficient
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Operator, Token};

    #[test]
    fn test_build_simple_expression() {
        let expr = ExpressionBuilder::new()
            .number(2.0)
            .operator(Operator::Add)
            .number(3.0)
            .build()
            .unwrap();
        assert_eq!(expr.tokens.len(), 3);
    }

    #[test]
    fn test_build_with_paren() {
        let expr = ExpressionBuilder::new()
            .number(2.0)
            .operator(Operator::Add)
            .open_paren()
            .number(3.0)
            .operator(Operator::Multiply)
            .number(4.0)
            .close_paren()
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(expr.tokens.len(), 7);
    }

    #[test]
    fn test_unmatched_close_paren() {
        let res = ExpressionBuilder::new()
            .number(1.0)
            .close_paren();
        assert!(res.is_err());
    }

    #[test]
    fn test_unmatched_open_paren() {
        let res = ExpressionBuilder::new()
            .number(1.0)
            .open_paren()
            .build();
        assert!(res.is_err());
    }

    #[test]
    fn test_empty_expression() {
        let res = ExpressionBuilder::new().build();
        assert!(res.is_err());
    }

    #[test]
    fn test_consecutive_operators() {
        let res = ExpressionBuilder::new()
            .number(1.0)
            .operator(Operator::Add)
            .operator(Operator::Multiply)
            .number(2.0)
            .build();
        assert!(res.is_err());
    }
}
