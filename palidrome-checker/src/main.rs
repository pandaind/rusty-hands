use std::io;

fn main() {
    println!("Palindrome Checker");
    let input = read_input("Enter a string to check if it's a palindrome:");
    let is_palindrome = check_palindrome(&input);
    if is_palindrome {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn check_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
                            .filter(|c| c.is_alphanumeric()) // Keep only alphanumeric characters
                            .collect::<Vec<char>>().into_iter() // Convert to iterator
                            .map(|c| c.to_ascii_lowercase()) // Convert to lowercase
                            .collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}