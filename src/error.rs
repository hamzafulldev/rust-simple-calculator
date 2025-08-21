#[derive(Debug)]
pub enum CalculatorError {
    InvalidInput(String),
    DivisionByZero,
    ParseError(String),
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalculatorError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CalculatorError::DivisionByZero => write!(f, "Error: Division by zero"),
            CalculatorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for CalculatorError {}