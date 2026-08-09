use std::io;

fn main() {
    println!("Enter a number to check if it's prime:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}