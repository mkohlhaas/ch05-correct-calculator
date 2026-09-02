#![allow(unused)]

// "Rather than creating a complex class hierarchy, we'll use Rust's enums (which provide sum type
// functionality) to represent our different token types."

// "Rust's enum system gives us polymorphism through variants rather than inheritance, and pattern
// matching provides exhaustive handling at compile time rather than virtual dispatch at runtime."

// "In many languages, factories are a pattern that we impose upon the language. For Rust, factories
// are the idiomatic and natural way to create type instances."

// token.rs - Core token types and factory methods

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number), // NOTE:hiccup is idiomatic Rust - The outer Number is the Token variant name; the inner is the wrapped Number enum type.
    Operator(Operator),
    Function(Function),
    Variable(String),
    OpenParen,
    CloseParen,
}

// ====================================== //
// Factory methods/constructors for Token //
// ====================================== //

impl Token {
    // Factory method for creating number tokens
    pub fn number(value: f64) -> Self {
        Self::Number(Number::new(value))
    }

    // Factory method for scientific notation
    pub fn scientific_number(value: f64) -> Self {
        Self::Number(Number::new_with_kind(value, NumberType::Scientific))
    }

    // Factory method for engineering notation
    pub fn engineering_number(value: f64) -> Self {
        Self::Number(Number::new_with_kind(value, NumberType::Engineering))
    }

    // Factory method for operators
    pub fn operator(op: Operator) -> Self {
        Self::Operator(op)
    }

    // Factory method for functions
    pub fn function(func: Function) -> Self {
        Self::Function(func)
    }

    // Factory method for variables
    // NOTE: `impl Into<String>` (accepts any type convertible to String)
    pub fn variable(name: impl Into<String>) -> Self {
        Self::Variable(name.into())
    }

    // Factory method for all tokens from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        // try parsing as a number first
        if let Ok(num) = s.parse::<f64>() {
            if s.contains('e') || s.contains('E') {
                return Ok(Self::scientific_number(num));
            }
            return Ok(Self::number(num));
        }

        match s {
            // Operators
            "+" => Ok(Self::operator(Operator::Add)),
            "-" => Ok(Self::operator(Operator::Subtract)),
            "*" => Ok(Self::operator(Operator::Multiply)),
            "/" => Ok(Self::operator(Operator::Divide)),
            "^" => Ok(Self::operator(Operator::Power)),
            // Functions
            "sin" => Ok(Self::function(Function::Sin)),
            "cos" => Ok(Self::function(Function::Cos)),
            "tan" => Ok(Self::function(Function::Tan)),
            "sqrt" => Ok(Self::function(Function::Sqrt)),
            // Parentheses
            "(" => Ok(Self::OpenParen),
            ")" => Ok(Self::CloseParen),
            // Variable
            name if name.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                Ok(Self::variable(name))
            }
            // Invalid token
            _ => Err(format!("Invalid token: {}", s)),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum NumberType {
    #[default]
    Decimal,
    Scientific,
    Engineering,
    // more kinds can be added …
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Number {
    pub value: f64,
    pub kind: NumberType,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            kind: Default::default(),
        }
    }

    pub fn new_with_kind(value: f64, kind: NumberType) -> Self {
        Self { value, kind }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            NumberType::Decimal => write!(f, "{}", self.value),
            NumberType::Scientific => write!(f, "{:e}", self.value),
            NumberType::Engineering => {
                let exp = self.value.abs().log10().floor();
                let adj_exp = (exp - exp % 3.0).floor();
                let coeff = self.value / 10_f64.powf(adj_exp);
                write!(f, "{}e{}", coeff, adj_exp)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Sqrt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_from_str_number() {
        let t = Token::from_str("42").unwrap();
        assert!(matches!(
            t,
            Token::Number(Number {
                value: 42.0,
                kind: NumberType::Decimal
            })
        ));
    }

    #[test]
    fn test_token_from_str_scientific() {
        let t = Token::from_str("1.23e4").unwrap();
        assert!(matches!(
            t,
            Token::Number(Number {
                value: 12300.0,
                kind: NumberType::Scientific
            })
        ));
    }

    #[test]
    fn test_token_from_str_operator() {
        assert!(matches!(
            Token::from_str("+").unwrap(),
            Token::Operator(Operator::Add)
        ));
    }

    #[test]
    fn test_token_from_str_function() {
        assert!(matches!(
            Token::from_str("sin").unwrap(),
            Token::Function(Function::Sin)
        ));
    }

    #[test]
    fn test_token_number_factory() {
        let t = Token::number(3.5);
        assert!(matches!(
            t,
            Token::Number(Number {
                value: 3.5,
                kind: NumberType::Decimal
            })
        ));
    }

    #[test]
    fn test_token_variable_factory() {
        let t = Token::variable("x");
        assert!(matches!(t, Token::Variable(s) if s == "x"));
    }

    #[test]
    fn test_number_format_engineering() {
        let n = Number::new_with_kind(1234.5, NumberType::Engineering);
        assert_eq!(n.to_string(), "1.2345e3");
    }

    #[test]
    fn test_invalid_token() {
        let res = Token::from_str("@");
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Invalid token"));
    }
}
