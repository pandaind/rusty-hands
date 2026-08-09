use std::{io, thread, time::Duration};
use std::io::Write;

fn main() {
    println!("Timer Tool");
    println!("Enter the time duration in format of hours:minutes:seconds (e.g., 1:30:00 for 1 hour and 30 minutes):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let time_parts: Vec<&str> = input.trim().split(':').collect();
    let hours = time_parts[0].parse::<u64>().expect("Please enter a valid hour");
    let minutes = time_parts[1].parse::<u64>().expect("Please enter a valid minute");
    let seconds = time_parts[2].parse::<u64>().expect("Please enter a valid second");

    let total_seconds = hours * 3600 + minutes * 60 + seconds;
    // Start the timer
    start_timer(total_seconds);
}

fn start_timer(total_seconds: u64) {
    for remaining in (0..=total_seconds).rev() {
        let hours = remaining / 3600;
        let minutes = (remaining % 3600) / 60;
        let seconds = remaining % 60;

        print!("\rTime remaining: {:02}:{:02}:{:02}", hours, minutes, seconds);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("\nTime's up!");
}
