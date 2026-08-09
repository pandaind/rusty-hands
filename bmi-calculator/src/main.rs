use std::io;

fn main() {
    println!("BMI Calculator");
    println!("Please enter your weight in kilograms:");
    let weight_input = read_input("Weight (kg):");
    println!("Please enter your height in meters:");
    let height_input = read_input("Height (m):");

    let bmi = calculate_bmi(weight_input, height_input);

    let category = categorize_bmi(bmi);
    println!("Your BMI is {:.2}, which is considered {}.", bmi, category);
}

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn categorize_bmi(bmi: f64) -> &'static str {
    match bmi {
        bmi if bmi < 18.5 => "underweight",
        bmi if bmi >= 18.5 && bmi < 24.9 => "normal weight",
        bmi if bmi >= 25.0 && bmi < 29.9 => "overweight",
        _ => "obese",
    }
}