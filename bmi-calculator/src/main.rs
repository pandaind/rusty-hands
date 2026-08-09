use std::io;

fn main() {
    println!("BMI Calculator");
    println!("Please enter your weight in kilograms:");
    let weight_input = read_input("Weight (kg):");
    println!("Please enter your height in meters:");
    let height_input = read_input("Height (m):");

    let bmi = weight_input / (height_input * height_input);
    println!("Your BMI is: {:.2}", bmi);

    match bmi {
        bmi if bmi < 18.5 => println!("You are underweight."),
        bmi if bmi >= 18.5 && bmi < 24.9 => println!("You have a normal weight."),
        bmi if bmi >= 25.0 && bmi < 29.9 => println!("You are overweight."),
        _ => println!("You are obese."),
    }
}

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}