use std::fs::File;
use std::io::Read;

fn main() {
    // Collect command line arguments into a vector
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("Counting words in file: {}", file_path);

    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", file_path, e);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file {}: {}", file_path, e);
        std::process::exit(1);
    }
    println!("File contents:\n{}", contents);

    let word_count = contents.split_whitespace().count();
    println!("Word count: {}", word_count);
}