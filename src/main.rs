// Correct Calculator - Chapter 5
// Main entry point demonstrating the calculator's features

mod builder;
mod config;
mod factory;
mod token;

use builder::ExpressionBuilder;
use config::CalculatorConfig;
use factory::{NumberToken, ScientificFactory, StandardTokenFactory, TokenFactory};
use token::{Function, Operator, Token};

fn main() {
    // Demonstrate Factory Methods
    let num_token = Token::number(42.0);
    let op_token = Token::operator(Operator::Add);
    let func_token = Token::function(Function::Sin);
    let var_token = Token::variable("x");

    println!(
        "Created tokens: {:?}, {:?}, {:?}, {:?}",
        num_token, op_token, func_token, var_token
    );

    // Demonstrate Factory from string
    match Token::from_str("3.14") {
        Ok(token) => println!("Parsed number: {:?}", token),
        Err(e) => println!("Error: {}", e),
    }

    println!("Token from String: {:?}", Token::from_str("3.14"));
    println!("Token from String: {:?}", Token::from_str("1.0e6"));
    println!("Token from String: {:?}", Token::from_str("x"));
    println!("Token from String: {:?}", Token::from_str("sin"));
    println!("Token from String: {:?}", Token::from_str("cos"));
    println!("Token from String: {:?}", Token::from_str("+"));

    // we need the extra spaces
    let tokens: Result<Vec<Token>, String> = "2 + ( 3 * 4 )"
        .split_whitespace()
        .map(Token::from_str)
        .collect();
    match tokens {
        Ok(expr) => println!("Valid expression: {:?}", expr),
        Err(e) => println!("Error: {}", e),
    }

    let tokens: Result<Vec<_>, _> = "2 + (3 * 4)"
        .split_whitespace()
        .map(Token::from_str)
        .collect();
    match tokens {
        Ok(expr) => println!("Valid expression: {:?}", expr),
        Err(e) => println!("Error: {}", e),
    }

    // Abstract Factory Demonstration
    let standard_factory = StandardTokenFactory;
    let sci_factory = ScientificFactory;

    let standard_num = standard_factory.create_number_token("123").unwrap();
    let sci_num = sci_factory.create_number_token("1.23e-4").unwrap();

    let standard_plus = standard_factory.create_operator_token("+");
    let scientific_plus = sci_factory.create_operator_token("+");

    println!("Standard number: {}", standard_num.format());
    println!("Scientific number: {}", sci_num.format());
    println!("Standard operator: {:?}", standard_plus.unwrap());
    println!("Scientific operator: {:?}", scientific_plus.unwrap());

    let expr1 = vec![
        Token::number(2.0),
        Token::operator(Operator::Add),
        Token::number(3.0),
    ];

    let expr2 = vec![Token::function(Function::Sin), Token::variable("x")];
    println!("Manual expression: {:?}", expr1);
    println!("Manual expression: {:?}", expr2);

    // Demonstrate Builder pattern
    // 2.0 + (3.0 * 4.0) = 14.0
    let expr3 = ExpressionBuilder::new()
        .number(2.0)
        .operator(Operator::Add)
        .open_paren()
        .number(3.0)
        .operator(Operator::Multiply)
        .number(4.0)
        .close_paren()
        .unwrap() // close_paren returns Result<Self, String>
        .build()
        .unwrap();

    println!("Built expression: {:?}", expr3);

    // Demonstrate configuration (alternative to Singleton)
    let default_config = CalculatorConfig::default();
    let sci_config = CalculatorConfig::scientific();

    println!("Default config: {:?}", default_config);
    println!("Scientific config: {:?}", sci_config);
}
