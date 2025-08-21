use std::io;
use crate::calculator::Calculator;

pub struct UserInterface {
    calculator: Calculator,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            calculator: Calculator::new(),
        }
    }

    pub fn run(&self) {
        self.print_welcome();
        
        loop {
            println!("\nEnter expression:");
            
            match self.get_user_input() {
                Ok(input) => {
                    let input = input.trim();
                    
                    if self.should_quit(&input) {
                        println!("Goodbye!");
                        break;
                    }
                    
                    self.process_expression(&input);
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                    break;
                }
            }
        }
    }

    fn print_welcome(&self) {
        println!("Simple Calculator");
        println!("Enter expressions like: 5 + 3, 10 - 2, 4 * 6, 15 / 3");
        println!("Type 'quit' or 'exit' to quit");
    }

    fn get_user_input(&self) -> Result<String, io::Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn should_quit(&self, input: &str) -> bool {
        input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit")
    }

    fn process_expression(&self, expression: &str) {
        match self.calculator.evaluate(expression) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("{}", error),
        }
    }
}

impl Default for UserInterface {
    fn default() -> Self {
        Self::new()
    }
}