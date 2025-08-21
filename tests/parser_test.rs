use rust_simple_calculator::{Parser, CalculatorError};

#[test]
fn test_parse_valid_expressions() {
    assert_eq!(Parser::parse_expression("2 + 3").unwrap(), 5.0);
    assert_eq!(Parser::parse_expression("10 - 4").unwrap(), 6.0);
    assert_eq!(Parser::parse_expression("3 * 4").unwrap(), 12.0);
    assert_eq!(Parser::parse_expression("15 / 3").unwrap(), 5.0);
    
    assert_eq!(Parser::parse_expression("2.5 + 3.7").unwrap(), 6.2);
    assert_eq!(Parser::parse_expression("-5 + 3").unwrap(), -2.0);
    
    assert_eq!(Parser::parse_expression("  8  *  2  ").unwrap(), 16.0);
}

#[test]
fn test_parse_invalid_expressions() {
    assert!(Parser::parse_expression("").is_err());
    assert!(Parser::parse_expression("2 +").is_err());
    assert!(Parser::parse_expression("+ 3").is_err());
    assert!(Parser::parse_expression("abc + 3").is_err());
    assert!(Parser::parse_expression("2 % 3").is_err());
    assert!(Parser::parse_expression("2 3").is_err());
}

#[test]
fn test_parse_division_by_zero() {
    match Parser::parse_expression("5 / 0") {
        Err(CalculatorError::DivisionByZero) => (),
        _ => panic!("Expected division by zero error"),
    }
    
    match Parser::parse_expression("10 / 0.0") {
        Err(CalculatorError::DivisionByZero) => (),
        _ => panic!("Expected division by zero error"),
    }
}