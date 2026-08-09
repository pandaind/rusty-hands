use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("Enter your choice (1 or 2): ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else {
        println!("Invalid choice. Please enter 1 or 2.");
    }
}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius: ");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read input");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{:.2}°C is equal to {:.2}°F", celsius, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit: ");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read input");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{:.2}°F is equal to {:.2}°C", fahrenheit, celsius);
}