use rust_simple_calculator::{Calculator, CalculatorError};

#[test]
fn test_calculator_evaluation() {
    let calc = Calculator::new();
    
    assert_eq!(calc.evaluate("5 + 5").unwrap(), 10.0);
    assert_eq!(calc.evaluate("20 - 8").unwrap(), 12.0);
    assert_eq!(calc.evaluate("6 * 7").unwrap(), 42.0);
    assert_eq!(calc.evaluate("100 / 4").unwrap(), 25.0);
}

#[test]
fn test_calculator_error_handling() {
    let calc = Calculator::new();
    
    assert!(calc.evaluate("").is_err());
    assert!(calc.evaluate("invalid").is_err());
    assert!(calc.evaluate("2 +").is_err());
    
    match calc.evaluate("1 / 0") {
        Err(CalculatorError::DivisionByZero) => (),
        _ => panic!("Expected division by zero error"),
    }
}

#[test]
fn test_calculator_default() {
    let calc = Calculator::default();
    assert_eq!(calc.evaluate("3 + 4").unwrap(), 7.0);
}