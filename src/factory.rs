#![allow(unused)]

// "Each calculator mode needs its own set of tokens, operators, and evaluation rules."

// "The Abstract Factory concept often maps more naturally to traits and associated types."

// "Instead of the complex inheritance hierarchies often seen in object-oriented languages, we get a
// clean, type-safe solution."

// What do you think about the application of the abstract factory pattern? Does it make any sense?

// Yes, conceptually it fits.
//
// TokenFactory creates matched families (StandardNumberToken + StandardOperatorToken vs.
// ScientificNumberToken + ScientificOperatorToken), which is exactly what Abstract Factory is for:
// ensuring a calculator uses consistent token types without hard-coding concrete classes. It makes
// sense here because the trait uses associated types to bind the family together, and Calculator<F>
// stays generic over the factory. The main caveat is that the concrete products are thin wrappers
// around Number/Operator; the pattern pays off more if the families diverge in behavior
// (formatting, precedence, evaluation). As a structural demonstration it works, but it’s slightly
// heavy for just parsing strings into an enum.

// "Traits combined with associated types allow us to ensure that all tokens from a given factory
// work together correctly. This gives us compile-time guarantees that we can't mix incompatible
// token types."

// factory.rs - Abstract Factory implementation

use crate::Token;
use crate::token::{Function, Number, NumberKind, Operator};
use std::fmt::{self, Display};

// //////////////////// //
// 1. Abstract Products //
// //////////////////// //

// We want to produce different kinds of Tokens!

// Trait for number tokens
pub trait NumberToken {
    fn value(&self) -> f64;
}

// Trait for operator tokens
pub trait OperatorToken {
    fn precedence(&self) -> u8;
    fn symbol(&self) -> &'static str;
}

// ==================== //
// 2. Concrete Products //
// ==================== //

// ------------------- //
// 2.1 Standard Tokens //
// ------------------- //

// for Standard Calculator mode

#[derive(Debug, PartialEq)]
pub struct StandardNumberToken(Number);

impl NumberToken for StandardNumberToken {
    fn value(&self) -> f64 {
        self.0.value
    }
}

impl Display for StandardNumberToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<StandardNumberToken> for Token {
    fn from(num: StandardNumberToken) -> Self {
        Token::Number(num.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct StandardOperatorToken(Operator);

impl OperatorToken for StandardOperatorToken {
    fn precedence(&self) -> u8 {
        match self.0 {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Power => 3,
        }
    }

    fn symbol(&self) -> &'static str {
        match self.0 {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
        }
    }
}

impl From<StandardOperatorToken> for Token {
    fn from(op: StandardOperatorToken) -> Self {
        Token::Operator(op.0)
    }
}

// ---------------------- //
// 2.2. Scientific Tokens //
// ---------------------- //

// for Scientific Calculator mode

#[derive(Debug, PartialEq)]
pub struct ScientificNumberToken(Number);

impl NumberToken for ScientificNumberToken {
    fn value(&self) -> f64 {
        self.0.value
    }
}

impl Display for ScientificNumberToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ScientificNumberToken> for Token {
    fn from(num: ScientificNumberToken) -> Self {
        Token::Number(num.0)
    }
}

#[derive(Debug, PartialEq)]
pub enum ScientificOperatorToken {
    Basic(Operator),
    Function(Function),
}

impl OperatorToken for ScientificOperatorToken {
    fn precedence(&self) -> u8 {
        match self {
            ScientificOperatorToken::Basic(op) => match op {
                Operator::Add | Operator::Subtract => 1,
                Operator::Multiply | Operator::Divide => 2,
                Operator::Power => 3,
            },
            ScientificOperatorToken::Function(_) => 4,
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            ScientificOperatorToken::Basic(op) => match op {
                Operator::Add => "+",
                Operator::Subtract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
                Operator::Power => "^",
            },
            ScientificOperatorToken::Function(func) => match func {
                Function::Sin => "sin",
                Function::Cos => "cos",
                Function::Tan => "tan",
                Function::Sqrt => "sqrt",
            },
        }
    }
}

impl From<ScientificOperatorToken> for Token {
    fn from(op: ScientificOperatorToken) -> Self {
        match op {
            ScientificOperatorToken::Basic(o) => Token::Operator(o),
            ScientificOperatorToken::Function(f) => Token::Function(f),
        }
    }
}

// =================== //
// 3. Abstract Factory //
// =================== //

// Abstract Factory trait with associated types as trait bounds!
// Abstract Factory pattern makes sense bc we only want matching Numbers and Operators!
// For example:
// Standard   Calculator uses StandardNumberTokens   with StandardOperatorTokens
// Scientific Calculator uses ScientificNumberTokens with ScientificOperatorTokens
// ...

pub trait TokenFactory {
    // type alias `Number` should be called NumberToken but this name is already used by the trait
    type Number: NumberToken + Into<Token>;
    type Operator: OperatorToken + Into<Token>;

    fn create_number_token(&self, s: &str) -> Result<Self::Number, String>;
    fn create_operator_token(&self, s: &str) -> Result<Self::Operator, String>;
}

// ===================== //
// 4. Concrete Factories //
// ===================== //

// -------------------------- //
// 4.1 Standard Token Factory //
// -------------------------- //

#[derive(Debug)]
pub struct StandardTokenFactory;

impl TokenFactory for StandardTokenFactory {
    type Number = StandardNumberToken;
    type Operator = StandardOperatorToken;

    fn create_number_token(&self, s: &str) -> Result<Self::Number, String> {
        match s.parse::<f64>() {
            Ok(value) => Ok(StandardNumberToken(Number::new(value))),
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    }

    fn create_operator_token(&self, s: &str) -> Result<Self::Operator, String> {
        match s {
            "+" => Ok(StandardOperatorToken(Operator::Add)),
            "-" => Ok(StandardOperatorToken(Operator::Subtract)),
            "*" => Ok(StandardOperatorToken(Operator::Multiply)),
            "/" => Ok(StandardOperatorToken(Operator::Divide)),
            "^" => Ok(StandardOperatorToken(Operator::Power)),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

// ---------------------------- //
// 4.2 Scientific Token Factory //
// ---------------------------- //

#[derive(Debug)]
pub struct ScientificFactory;

impl TokenFactory for ScientificFactory {
    type Number = ScientificNumberToken;
    type Operator = ScientificOperatorToken;

    fn create_number_token(&self, s: &str) -> Result<Self::Number, String> {
        // Handle both scientific and standard notation
        match s.parse::<f64>() {
            Ok(value) => {
                let format = if s.contains('e') || s.contains('E') {
                    NumberKind::Scientific
                } else {
                    NumberKind::Decimal
                };
                Ok(ScientificNumberToken(Number::new_with_kind(value, format)))
            }
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    }

    fn create_operator_token(&self, s: &str) -> Result<Self::Operator, String> {
        // Scientific calculator supports functions
        match s {
            "+" => Ok(ScientificOperatorToken::Basic(Operator::Add)),
            "-" => Ok(ScientificOperatorToken::Basic(Operator::Subtract)),
            "*" => Ok(ScientificOperatorToken::Basic(Operator::Multiply)),
            "/" => Ok(ScientificOperatorToken::Basic(Operator::Divide)),
            "^" => Ok(ScientificOperatorToken::Basic(Operator::Power)),
            "sin" => Ok(ScientificOperatorToken::Function(Function::Sin)),
            "cos" => Ok(ScientificOperatorToken::Function(Function::Cos)),
            "tan" => Ok(ScientificOperatorToken::Function(Function::Tan)),
            "sqrt" => Ok(ScientificOperatorToken::Function(Function::Sqrt)),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

// ============== //
// 5. Client Code //
// ============== //

// Our Calculator only parses input into self.expression (Vec<Token>).
// It has no evaluate or computation method, so it does not calculate any result yet.

// NOTE: here is our Calculator, but not used!
struct Calculator<F: TokenFactory> {
    factory: F,
    expression: Vec<Token>,
}

impl<F: TokenFactory> Calculator<F> {
    pub fn new(factory: F) -> Self {
        Self {
            factory,
            expression: Vec::new(),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Vec<Token>, String> {
        for token in input.split_whitespace() {
            if token == "(" {
                self.expression.push(Token::OpenParen);
                continue;
            }
            if token == ")" {
                self.expression.push(Token::CloseParen);
                continue;
            }
            // Try operator first ...
            if let Ok(op) = self.factory.create_operator_token(token) {
                self.expression.push(op.into());
                continue;
            }
            // ... must be a number then.
            let num = self.factory.create_number_token(token)?;
            self.expression.push(num.into());
        }
        Ok(self.expression.clone())
    }
}

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_factory_number() {
        let f = StandardTokenFactory;
        let n = f.create_number_token("99").unwrap();
        assert_eq!(n.value(), 99.0);
    }

    #[test]
    fn test_standard_factory_operator() {
        let f = StandardTokenFactory;
        let op = f.create_operator_token("*").unwrap();
        assert_eq!(op.symbol(), "*");
    }

    #[test]
    fn test_scientific_factory_scientific_notation() {
        let f = ScientificFactory;
        let n = f.create_number_token("1.23e-4").unwrap();
        assert_eq!(n.value(), 0.000123);
    }

    #[test]
    fn test_scientific_factory_function_op() {
        let f = ScientificFactory;
        let op = f.create_operator_token("sin").unwrap();
        assert_eq!(op.symbol(), "sin");
    }

    #[test]
    fn test_parse_standard_simple() {
        let mut calc = Calculator::new(StandardTokenFactory);
        calc.parse("2 + 3").unwrap();
        assert_eq!(calc.expression.len(), 3);
    }

    #[test]
    fn test_parse_standard_with_paren() {
        let mut calc = Calculator::new(StandardTokenFactory);
        calc.parse("2 + ( 3 * 4 )").unwrap();
        assert_eq!(calc.expression.len(), 7);
    }

    #[test]
    fn test_parse_scientific_with_function() {
        let mut calc = Calculator::new(ScientificFactory);
        calc.parse("sin + 1").unwrap();
        assert_eq!(calc.expression.len(), 3);
    }

    #[test]
    fn test_parse_invalid_token_fails() {
        let mut calc = Calculator::new(StandardTokenFactory);
        assert!(calc.parse("2 + @").is_err());
    }
}
