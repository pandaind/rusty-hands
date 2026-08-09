use std::io;

fn main() {
    println!("Enter the number of Fibonacci numbers to generate:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Please enter a valid number");

    let fibonacci_sequence = generate_fibonacci(n);

    println!("Fibonacci sequence up to {} terms:", n);
    for number in fibonacci_sequence {
        println!("{}", number);
    }
}

fn generate_fibonacci(n: usize) -> Vec<u64> {
    let mut sequence = Vec::with_capacity(n);
    if n == 0 {
        return sequence;
    }
    sequence.push(0);
    if n == 1 {
        return sequence;
    }
    sequence.push(1);
    for i in 2..n {
        let next_number = sequence[i - 1] + sequence[i - 2];
        sequence.push(next_number);
    }
    sequence
}