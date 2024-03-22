use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handling incorrect number of arguments
    if args.len() != 4 {
        println!("Usage: {} <operation> <operand1> <operand2>", args[0]);
        return;
    }

    // Parsing arg inputs
    let operation = &args[1];
    let operand1: f64 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Operand 1 must be a valid number.");
            return;
        }
    };
    let operand2: f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Operand 2 must be a valid number.");
            return;
        }
    };

    // Pattern matching operations
    let result = match operation.as_str() {
        "add" => operand1 + operand2,
        "subtract" => operand1 - operand2,
        "multiply" => operand1 * operand2,
        "divide" => {
            if operand2 == 0.0 {
                println!("Error: Division by zero.");
                return;
            }
            operand1 / operand2
        }
        _ => {
            println!("Error: Invalid operation. Valid operations are 'add', 'subtract', 'multiply', 'divide'.");
            return;
        }
    };

    println!("Result: {}", result);
}
