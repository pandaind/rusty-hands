use std::env;
use std::fs;
use serde_json::Value;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_json_file>", args[0]);
        std::process::exit(1);
    }

    // Read the JSON file
    let file_path = &args[1];
    match fs:: read_to_string(file_path) {
        Ok(file_content) => {
            // Parse the JSON content
            match serde_json::from_str::<Value>(&file_content) {
                Ok(json_value) => {
                    // Print the parsed JSON value
                    println!("{:#?}", json_value);
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
            std::process::exit(1);
        }
    }
}