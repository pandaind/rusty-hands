use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Available operations: +, -, *, /");
    println!("Enter your calculation in the format: number1 operator number2 (e.g., 5 + 3)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        println!("Invalid input format. Please enter in the format: number1 operator number2");
        return; 
    }

    let num1: f64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", parts[0]);
            return;
        }
    };

    let operator = parts[1];

    let num2: f64 = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", parts[2]);
            return;
        }
    };

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            num1 / num2
        },
        _ => {
            println!("Invalid operator: {}. Please use one of +, -, *, /", operator);
            return;
        }
    };

    println!("Result: {}", result);

}