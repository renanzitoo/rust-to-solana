use clap::Parser;
use sha2::{Digest, Sha256, Sha512};
use std::fs;

#[derive(Parser)]
struct Args{
    #[arg(long)]
    text: Option<String>,

    #[arg(long)]
    file: Option<String>,

    #[arg(long, default_value = "sha256")]
    algo: String,
}

fn hash_text(text: &str, algo: &str) -> String{
    match algo{
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(text.as_bytes());
            format!("{:x}", hasher.finalize())
        },
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(text.as_bytes());
            format!("{:x}", hasher.finalize())
        },
        _ => panic!("Unsupported algorithm"),
    }
}

fn hash_file(path: &str, algo: &str) -> String{
    let content = fs::read_to_string(path).expect("Failed to read file");
    hash_text(&content, algo)
}

fn main(){
    let args = Args::parse();

    if let Some(text) = args.text {
        println!("{}", hash_text(&text, &args.algo));
    }
    if let Some(file) = args.file {
        println!("{}", hash_file(&file, &args.algo));
    }
    
}