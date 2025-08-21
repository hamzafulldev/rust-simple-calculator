use crate::error::CalculatorError;

pub struct Operations;

impl Operations {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    pub fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b == 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
}