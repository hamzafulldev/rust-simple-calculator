use crate::error::CalculatorError;
use crate::parser::Parser;

pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn evaluate(&self, expression: &str) -> Result<f64, CalculatorError> {
        Parser::parse_expression(expression)
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}