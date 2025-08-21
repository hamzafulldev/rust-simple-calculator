use rust_simple_calculator::{Operations, CalculatorError};

#[test]
fn test_addition() {
    assert_eq!(Operations::add(2.0, 3.0), 5.0);
    assert_eq!(Operations::add(-2.0, 3.0), 1.0);
    assert_eq!(Operations::add(0.0, 0.0), 0.0);
    assert_eq!(Operations::add(2.5, 3.7), 6.2);
}

#[test]
fn test_subtraction() {
    assert_eq!(Operations::subtract(5.0, 3.0), 2.0);
    assert_eq!(Operations::subtract(3.0, 5.0), -2.0);
    assert_eq!(Operations::subtract(0.0, 0.0), 0.0);
    assert_eq!(Operations::subtract(10.5, 2.3), 8.2);
}

#[test]
fn test_multiplication() {
    assert_eq!(Operations::multiply(2.0, 3.0), 6.0);
    assert_eq!(Operations::multiply(-2.0, 3.0), -6.0);
    assert_eq!(Operations::multiply(0.0, 5.0), 0.0);
    assert_eq!(Operations::multiply(2.5, 4.0), 10.0);
}

#[test]
fn test_division() {
    assert_eq!(Operations::divide(6.0, 2.0).unwrap(), 3.0);
    assert_eq!(Operations::divide(5.0, 2.0).unwrap(), 2.5);
    assert_eq!(Operations::divide(10.0, 4.0).unwrap(), 2.5);
    
    match Operations::divide(5.0, 0.0) {
        Err(CalculatorError::DivisionByZero) => (),
        _ => panic!("Expected division by zero error"),
    }
}