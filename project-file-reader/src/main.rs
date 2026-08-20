// Build a Command-Line Interface (CLI) tool in Rust that reads a file and prints its contents.
// Add optional features like showing line numbers and 
// searching for keywords within the file.
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path> [--line-numbers] [--search <keyword>]", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let show_line_numbers = args.contains(&"--line-numbers".to_string());
    let search_keyword = args.iter().position(|arg| arg == "--search").map(|i| args.get(i + 1)).flatten();

    match read_file(file_path, show_line_numbers, search_keyword) {
        Ok(_) => (),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn read_file(file_path: &str, show_line_numbers: bool, search_keyword: Option<&String>) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if let Some(keyword) = search_keyword {
            if !line.contains(keyword) {
                continue;
            }
        }
        if show_line_numbers {
            println!("{}: {}", index + 1, line);
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}