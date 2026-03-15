use rand::Rng;
use std::{char, env};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    #[arg(short, long, default_value_t = String::from("complete"))]
    charset: String,
}


const COMPLETECHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

const ALPHANUMERICCHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789";

const ALPHABETICCHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz";


fn generate_password(length: usize, charset: &[u8]) -> String {
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    password
}

fn main() {
    let args = Args::parse();
    let password = generate_password(args.length, match args.charset.as_str() {
        "complete" => COMPLETECHARSET,
        "alphanumeric" => ALPHANUMERICCHARSET,
        "alphabetic" => ALPHABETICCHARSET,
        _ => panic!("Invalid charset"),
    });
    println!("Generated password: {}", password);
}
