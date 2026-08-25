#![allow(unused)]

// "Each calculator mode needs its own set of tokens, operators, and evaluation rules."

// "The Abstract Factory concept often maps more naturally to traits and associated types."

// "Instead of the complex inheritance hierarchies often seen in object-oriented languages, we get a
// clean, type-safe solution."

// factory.rs - Abstract Factory implementation

use crate::Token;
use crate::token::{Function, Number, NumberFormat, Operator};

// //////////////////// //
// 1. Abstract Products //
// //////////////////// //

// We want to produce Tokens!

// NOTE: These traits are useless!
// Trait for number tokens
pub trait NumberToken {
    fn value(&self) -> f64;
    fn format(&self) -> String;
}

// Trait for operator tokens
pub trait OperatorToken {
    fn precedence(&self) -> u8;
    fn symbol(&self) -> &'static str;
}

// //////////////////// //
// 2. Concrete Products //
// //////////////////// //

// /////////////////////// //
// 2.1 Standard Calculator //
// /////////////////////// //

// Standard calculator implementation
#[derive(Debug, Clone, PartialEq)]
pub struct StandardNumberToken(pub Number);
impl NumberToken for StandardNumberToken {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn format(&self) -> String {
        self.0.format()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StandardOperatorToken(pub Operator);
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

// ////////////////////////// //
// 2.2. Scientific calculator //
// ////////////////////////// //

// Scientific calculator implementation
#[derive(Debug, Clone, PartialEq)]
pub struct ScientificNumberToken(pub Number);
impl NumberToken for ScientificNumberToken {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn format(&self) -> String {
        // Scientific calculator prefers scientific notation by default
        match self.0.format {
            NumberFormat::Decimal => format!("{:e}", self.0.value),
            _ => self.0.format(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

// /////////////////// //
// 3. Abstract Factory //
// /////////////////// //

// Abstract Factory trait with associated types
pub trait TokenFactory {
    type Number: NumberToken;
    type Operator: OperatorToken;

    fn create_number_token(&self, s: &str) -> Result<Self::Number, String>;
    fn create_operator_token(&self, s: &str) -> Result<Self::Operator, String>;
}

// ///////////////////// //
// 4. Concrete Factories //
// ///////////////////// //

// ////////////////////////// //
// 4.1 Standard Token Factory //
// ////////////////////////// //

#[derive(Debug, Clone, Copy)]
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

// //////////////////////////// //
// 4.2 Scientific Token Factory //
// //////////////////////////// //

#[derive(Debug, Clone, Copy)]
pub struct ScientificFactory;

impl TokenFactory for ScientificFactory {
    type Number = ScientificNumberToken;
    type Operator = ScientificOperatorToken;

    fn create_number_token(&self, s: &str) -> Result<Self::Number, String> {
        // Handle both scientific and standard notation
        match s.parse::<f64>() {
            Ok(value) => {
                let format = if s.contains('e') || s.contains('E') {
                    NumberFormat::Scientific
                } else {
                    NumberFormat::Decimal
                };
                Ok(ScientificNumberToken(Number::new_with_format(
                    value, format,
                )))
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

// ////////////// //
// 5. Client Code //
// ////////////// //

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

    // TODO: parse is wrongly implemented
    // pub fn parse(&mut self, input: &str) -> Result<(), String> {
    //   for token in input.split_whitespace() {
    //     // Try operator first
    //     if let Ok(op) = self.factory.create_operator_token(token) {
    //       self.expression.push(Token::Operator(op));
    //       continue;
    //     }
    //     // Must be a number then
    //     let num = self.factory.create_number_token(token)?;
    //     self.expression.push(Token::Number(num));
    //   }
    //   Ok(())
    // }
}
