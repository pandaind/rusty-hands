 use std::io;
 use rand::Rng;

fn main() {
    println!("Rock-Paper-Scissors Game");

    loop {
        println!("Enter your choice (rock, paper, scissors or quit):");
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read input");

        match user_choice.trim().to_lowercase().as_str() {
            "rock" | "paper" | "scissors" => {
                println!("You chose: {}", user_choice);
                let mut rng = rand::rng();
                let choices = ["rock", "paper", "scissors"];
                let computer_choice = choices[rng.random_range(0..choices.len())];
                println!("Computer's choice: {}", computer_choice);
            }
            "quit" => {
                println!("Thanks for playing!");
                break;
            }
            _ => {
                println!("Invalid choice. Please choose rock, paper, or scissors.");
                return;
            }
        }
    }
}