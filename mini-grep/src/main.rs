use std::env;
use std::fs;

use mini_grep::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <query> <file>", args[0]);
        std::process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Error reading file");

    let results = search(query, &contents);

    for line in results {
        println!("{}", line);
    }
}
