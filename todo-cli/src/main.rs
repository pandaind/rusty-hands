use std::io;

// Program entry point
fn main() {
    println!("Welcome to the Todo CLI!");
    println!("Please enter a command (add, list, complete, delete):");
    println!("Type 'exit' to quit the application.");

    // Create an empty vector to store our tasks.
    //
    // Vec<String> means:
    //   Vec   -> a growable collection
    //   String -> each item in the collection is a String
    //
    // `mut` is required because we will add/delete/modify tasks.
    let mut tasks: Vec<String> = Vec::new();

    // Keep asking the user for commands until they type "exit".
    loop {
        // Create an empty String to hold the user's input.
        //
        // `mut` is required because read_line() will modify this String
        // by putting the user's input inside it.
        let mut input = String::new();

        // Read one line from standard input (keyboard).
        //
        // `&mut input` means:
        // "Give read_line() a mutable reference to input."
        //
        // The function does NOT take ownership of input.
        // It temporarily borrows input and modifies it.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // read_line() also captures the Enter key (`\n`).
        //
        // trim() removes whitespace and the newline.
        //
        // IMPORTANT:
        // `command` is a &str (string slice), not a new String.
        // It is borrowing data from `input`.
        let command = input.trim();

        // `match` compares `command` against different patterns.
        //
        // Rust requires match to handle every possible case.
        // `_` is the catch-all pattern.
        match command {
            // User entered "add"
            "add" => {
                // Ask the user for the task description.
                //
                // read_input() returns an owned String.
                let task = read_input("Enter the task description:");

                // Pass a mutable reference to tasks.
                //
                // `&mut tasks` means:
                // "Temporarily allow add_task() to modify tasks."
                //
                // tasks itself still belongs to main().
                add_task(&mut tasks, &task);

                println!("Task added: {}", task);
            }

            // User entered "list"
            "list" => {
                // We only need to READ tasks.
                //
                // Therefore we pass a normal reference: `&tasks`
                // instead of a mutable reference: `&mut tasks`.
                list_tasks(&tasks);
            }

            // User entered "complete"
            "complete" => {
                // Ask the user which task they want to complete.
                //
                // read_input() returns String.
                // `&` temporarily borrows that String.
                let task_number =
                    parse_task_number(&read_input(
                        "Enter the task number to complete:",
                    ));

                // parse_task_number() returns Option<usize>.
                //
                // Option can contain:
                //
                // Some(value) -> we successfully got a number
                // None         -> input was not a valid number
                //
                // `if let Some(n)` extracts the number from Some().
                if let Some(n) = task_number {
                    complete_task(&mut tasks, n);

                    println!(
                        "Task number {} marked as completed.",
                        n
                    );
                } else {
                    println!("Please enter a valid number.");
                }
            }

            // User entered "delete"
            "delete" => {
                // Ask which task should be deleted.
                let task_number =
                    parse_task_number(&read_input(
                        "Enter the task number to delete:",
                    ));

                // Check whether parsing was successful.
                if let Some(n) = task_number {
                    // We need `&mut tasks` because delete_task()
                    // will modify the vector.
                    delete_task(&mut tasks, n);

                    println!("Task number {} deleted.", n);
                } else {
                    println!("Please enter a valid number.");
                }
            }

            // User entered "exit"
            "exit" => {
                println!("Exiting Todo CLI. Goodbye!");

                // `break` exits the loop.
                break;
            }

            // `_` matches anything that wasn't handled above.
            //
            // For example:
            // hello
            // xyz
            // update
            // etc.
            _ => {
                println!("Unknown command. Please try again.");
            }
        }
    }
}


// ------------------------------------------------------------
// ADD TASK
// ------------------------------------------------------------

fn add_task(tasks: &mut Vec<String>, task: &str) {

    // `tasks` is a mutable reference to the original Vec.
    //
    // Because we have `&mut`, we are allowed to modify it.
    //
    // `push()` adds a new item to the vector.
    //
    // `task` is &str (borrowed string data).
    // `to_string()` creates an owned String from it.
    //
    // Why?
    //
    // Vec<String> needs to OWN the String it stores.
    tasks.push(task.to_string());
}


// ------------------------------------------------------------
// LIST TASKS
// ------------------------------------------------------------

fn list_tasks(tasks: &Vec<String>) {

    // `.iter()` creates an iterator over the vector.
    //
    // It gives us references to each task rather than
    // transferring ownership of the tasks.
    //
    // `.enumerate()` gives us:
    //
    //   index + item
    //
    // Example:
    //
    // 0 -> "Learn Rust"
    // 1 -> "Build Todo App"
    //
    for (i, task) in tasks.iter().enumerate() {

        // i starts from 0.
        //
        // Humans normally number tasks starting from 1,
        // so we use i + 1.
        println!("{}. {}", i + 1, task);
    }
}


// ------------------------------------------------------------
// COMPLETE TASK
// ------------------------------------------------------------

fn complete_task(tasks: &mut Vec<String>, task_number: usize) {

    // Check that the task number is valid.
    //
    // Users enter:
    //
    // 1 -> first task
    // 2 -> second task
    //
    // But Vec indexes start at 0:
    //
    // 0 -> first task
    // 1 -> second task
    //
    // Therefore we need task_number - 1 later.
    if task_number > 0 && task_number <= tasks.len() {

        // Get the task at the requested index.
        //
        // `task_number - 1` converts the user's 1-based
        // number into Rust's 0-based vector index.
        //
        // Example:
        //
        // user enters 1
        // 1 - 1 = 0
        //
        // user enters 3
        // 3 - 1 = 2
        //
        // `format!()` creates a NEW String.
        //
        // Example:
        //
        // "Learn Rust"
        //
        // becomes:
        //
        // "Learn Rust (completed)"
        tasks[task_number - 1] =
            format!("{} (completed)", tasks[task_number - 1]);
    }
}


// ------------------------------------------------------------
// DELETE TASK
// ------------------------------------------------------------

fn delete_task(tasks: &mut Vec<String>, task_number: usize) {

    // Make sure the requested task exists.
    if task_number > 0 && task_number <= tasks.len() {

        // Remove the task from the vector.
        //
        // `remove()` expects a 0-based index.
        //
        // So if the user enters:
        //
        // 1 -> remove index 0
        // 2 -> remove index 1
        //
        tasks.remove(task_number - 1);
    }
}


// ------------------------------------------------------------
// READ USER INPUT
// ------------------------------------------------------------

fn read_input(prompt: &str) -> String {

    // Print the message asking the user for input.
    //
    // `prompt: &str` means this function BORROWS
    // the string passed to it.
    //
    // It doesn't take ownership of the prompt.
    println!("{}", prompt);

    // Create an empty String.
    let mut input = String::new();

    // Read input from keyboard.
    //
    // `&mut input` gives stdin temporary permission
    // to modify our String.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Remove whitespace/newline and create an owned String.
    //
    // `trim()` -> &str
    // `to_string()` -> String
    //
    // The returned String belongs to the caller.
    input.trim().to_string()
}


// ------------------------------------------------------------
// PARSE TASK NUMBER
// ------------------------------------------------------------

fn parse_task_number(input: &str) -> Option<usize> {

    // Try to convert the input string into a usize.
    //
    // For example:
    //
    // "10" -> Ok(10)
    //
    // "abc" -> Err(...)
    //
    // parse() returns a Result.
    match input.trim().parse() {

        // Parsing succeeded.
        //
        // `n` contains the parsed usize.
        //
        // We wrap it inside Some().
        Ok(n) => Some(n),

        // Parsing failed.
        //
        // `_` means we don't care about the actual error.
        //
        // Return None to tell the caller:
        // "There is no valid number."
        Err(_) => None,
    }
}