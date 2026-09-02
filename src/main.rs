// Correct Calculator - Chapter 5
// Main entry point demonstrating the calculator's features

#![allow(unused_imports)]

mod builder;
mod config;
mod factory;
mod token;

// for testing
use builder::ExpressionBuilder;
use config::CalculatorConfig;
use factory::{NumberToken, OperatorToken, ScientificFactory, StandardTokenFactory, TokenFactory};
use token::{Function, Operator, Token};

fn main() {
    println!("Nothing to see here!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_factory_methods() {
        let num_token = Token::number(42.0);
        assert!(matches!(num_token, Token::Number(_)));
        let op_token = Token::operator(Operator::Add);
        assert!(matches!(op_token, Token::Operator(Operator::Add)));
        let func_token = Token::function(Function::Sin);
        assert!(matches!(func_token, Token::Function(Function::Sin)));
        let var_token = Token::variable("x");
        assert!(matches!(var_token, Token::Variable(s) if s == "x"));
    }

    #[test]
    fn test_token_from_str() {
        assert!(matches!(Token::from_str("3.14").unwrap(), Token::Number(_)));
        assert!(matches!(Token::from_str("x").unwrap(), Token::Variable(s) if s == "x"));
        assert!(matches!(
            Token::from_str("1.0e6").unwrap(),
            Token::Number(_)
        ));
        assert!(matches!(
            Token::from_str("sin").unwrap(),
            Token::Function(Function::Sin)
        ));
        assert!(matches!(
            Token::from_str("cos").unwrap(),
            Token::Function(Function::Cos)
        ));
        assert!(matches!(
            Token::from_str("+").unwrap(),
            Token::Operator(Operator::Add)
        ));
    }

    #[test]
    fn test_expression_parsing_with_spaces() {
        let tokens: Result<Vec<Token>, String> = "2 + ( 3 * 4 )"
            .split_whitespace()
            .map(Token::from_str)
            .collect();
        assert!(tokens.is_ok());
        assert_eq!(tokens.unwrap().len(), 7);
    }

    #[test]
    fn test_expression_with_invalid_token_foo_at_me() {
        let res: Result<Vec<Token>, String> = "2 + foo@me + ( 3 * 4 )"
            .split_whitespace()
            .map(Token::from_str)
            .collect();
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Invalid token"));
    }

    #[test]
    fn test_expression_2_plus_foo_plus_3_times_4() {
        let res: Result<Vec<Token>, String> = "2 + foo + ( 3 * 4 )"
            .split_whitespace()
            .map(Token::from_str)
            .collect();
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 9);
    }

    #[test]
    fn test_abstract_factory_standard() {
        let standard_factory = StandardTokenFactory;
        let standard_num = standard_factory.create_number_token("123").unwrap();
        assert_eq!(standard_num.value(), 123.0);
        let standard_plus = standard_factory.create_operator_token("+").unwrap();
        assert_eq!(standard_plus.symbol(), "+");
    }

    #[test]
    fn test_abstract_factory_scientific() {
        let sci_factory = ScientificFactory;
        let sci_num = sci_factory.create_number_token("1.23e-4").unwrap();
        assert_eq!(sci_num.value(), 0.000123);
        let scientific_plus = sci_factory.create_operator_token("+").unwrap();
        assert_eq!(scientific_plus.symbol(), "+");
    }

    #[test]
    fn test_manual_expressions() {
        let expr1 = [
            Token::number(2.0),
            Token::operator(Operator::Add),
            Token::number(3.0),
        ];
        assert_eq!(expr1.len(), 3);

        let expr2 = [Token::function(Function::Sin), Token::variable("x")];
        assert_eq!(expr2.len(), 2);
    }

    #[test]
    fn test_builder_expression() {
        let expr3 = ExpressionBuilder::new()
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
        assert!(!format!("{:?}", expr3).is_empty());
    }

    #[test]
    fn test_config_default_and_scientific() {
        let default_config = CalculatorConfig::default();
        assert_eq!(default_config.precision, 10);
        let sci_config = CalculatorConfig::scientific();
        assert_eq!(sci_config.precision, 15);
        assert!(matches!(sci_config.notation, token::NumberKind::Scientific));
    }
}
