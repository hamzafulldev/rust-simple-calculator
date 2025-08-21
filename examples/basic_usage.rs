use rust_simple_calculator::{Calculator, Operations, Parser};

fn main() {
    println!("=== Calculator Examples ===\n");

    // Using the Calculator struct
    println!("1. Using Calculator:");
    let calc = Calculator::new();
    let expressions = ["5 + 3", "10 - 2", "4 * 6", "15 / 3", "7.5 + 2.5"];
    
    for expr in expressions {
        match calc.evaluate(expr) {
            Ok(result) => println!("   {} = {}", expr, result),
            Err(e) => println!("   Error: {}", e),
        }
    }

    // Using Operations directly
    println!("\n2. Using Operations directly:");
    println!("   Operations::add(10.0, 5.0) = {}", Operations::add(10.0, 5.0));
    println!("   Operations::subtract(10.0, 3.0) = {}", Operations::subtract(10.0, 3.0));
    println!("   Operations::multiply(4.0, 7.0) = {}", Operations::multiply(4.0, 7.0));
    
    match Operations::divide(20.0, 4.0) {
        Ok(result) => println!("   Operations::divide(20.0, 4.0) = {}", result),
        Err(e) => println!("   Division error: {}", e),
    }

    // Using Parser directly
    println!("\n3. Using Parser directly:");
    let test_expressions = ["12 + 8", "25 - 10", "3 * 9"];
    
    for expr in test_expressions {
        match Parser::parse_expression(expr) {
            Ok(result) => println!("   Parser::parse_expression(\"{}\") = {}", expr, result),
            Err(e) => println!("   Parse error: {}", e),
        }
    }

    // Error handling examples
    println!("\n4. Error handling:");
    let error_expressions = ["10 / 0", "invalid input", "+ 5"];
    
    for expr in error_expressions {
        match calc.evaluate(expr) {
            Ok(result) => println!("   {} = {}", expr, result),
            Err(e) => println!("   {} -> Error: {}", expr, e),
        }
    }
}