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
                let result = determine_winner(user_choice.trim().to_lowercase().as_str(), computer_choice);
                println!("Result: {}", result);
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

fn determine_winner(user_choice: &str, computer_choice: &str) -> &'static str {
    if user_choice == computer_choice {
        "It's a tie!"
    } else if (user_choice == "rock" && computer_choice == "scissors")
        || (user_choice == "paper" && computer_choice == "rock")
        || (user_choice == "scissors" && computer_choice == "paper")
    {
        "You win!"
    } else {
        "Computer wins!"
    }
}