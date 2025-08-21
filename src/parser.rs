use crate::error::CalculatorError;
use crate::operations::Operations;

pub struct Parser;

impl Parser {
    pub fn parse_expression(input: &str) -> Result<f64, CalculatorError> {
        let input = input.trim();
        
        if input.is_empty() {
            return Err(CalculatorError::InvalidInput("Empty input".to_string()));
        }

        let operators = ['+', '-', '*', '/'];
        let mut operator_pos = None;
        let mut operator = None;

        for (i, ch) in input.char_indices() {
            if operators.contains(&ch) {
                if i == 0 && ch == '-' {
                    continue;
                }
                operator_pos = Some(i);
                operator = Some(ch);
                break;
            }
        }

        let (a_str, op, b_str) = match (operator_pos, operator) {
            (Some(pos), Some(op)) => {
                let a_str = &input[..pos].trim();
                let b_str = &input[pos + 1..].trim();
                (a_str, op, b_str)
            }
            _ => return Err(CalculatorError::InvalidInput("No operator found".to_string())),
        };

        let a = a_str.parse::<f64>()
            .map_err(|_| CalculatorError::ParseError(format!("Invalid number: {}", a_str)))?;
        let b = b_str.parse::<f64>()
            .map_err(|_| CalculatorError::ParseError(format!("Invalid number: {}", b_str)))?;

        match op {
            '+' => Ok(Operations::add(a, b)),
            '-' => Ok(Operations::subtract(a, b)),
            '*' => Ok(Operations::multiply(a, b)),
            '/' => Operations::divide(a, b),
            _ => Err(CalculatorError::InvalidInput(format!("Unsupported operator: {}", op))),
        }
    }
}