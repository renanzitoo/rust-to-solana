use std::env;
use std::fs;
use serde::{Serialize, Deserialize};
use rand::Rng;

const FILE_PATH: &str = "urls.json";

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    short: String,
    long: String,
    user: String,
}

fn load_urls() -> Vec<Url> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_urls(urls: &Vec<Url>) {
    let content = serde_json::to_string(urls).expect("Failed to serialize URLs");
    fs::write(FILE_PATH, content).expect("Failed to write URLs to file");
}

fn generate_short_url() -> String {
    let mut rng = rand::thread_rng();
    let short: String = (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            charset[idx] as char
        })
        .collect();
    short
}

fn create_url(long:String, user: String) -> Url {
    Url {
        short: generate_short_url(),
        long,
        user,
    }
}

fn get_url(urls: &Vec<Url>, short: &str) -> Option<String> {
    urls.iter().find(|url| url.short == short).map(|url| url.long.clone())
}

fn remove_url(urls: &mut Vec<Url>, short: &str) {
    urls.retain(|url| url.short != short);
}

fn list_urls(urls: &Vec<Url>, user: &str) {
    for url in urls.iter().filter(|url| url.user == user) {
        println!("{} -> {}", url.short, url.long);
    }
}

fn main() {
    let mut urls: Vec<Url>  = load_urls();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: url_shortener <command> [arguments]");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 4 {
                println!("Usage: url_shortener add <long_url> <user>");
                return;
            }
            let url = create_url(args[2].clone(), args[3].clone());
            urls.push(url);
            save_urls(&urls);
            println!("URL added: {} -> {}", args[2], urls.last().unwrap().short);
        }
        "remove" => {
            if args.len() < 3 {
                println!("Usage: url_shortener remove <short_url>");
                return;
            }
            remove_url(&mut urls, &args[2]);
            save_urls(&urls);
            println!("URL removed: {}", args[2]);
        }
        "list" => {
            if args.len() < 3 {
                println!("Usage: url_shortener list <user>");   
                return;
            }
            list_urls(&urls, &args[2]);
        }
        "get" => {
            if args.len() < 3 {
                println!("Usage: url_shortener get <short_url>");
                return;
            }
            match get_url(&urls, &args[2]) {
                Some(long) => println!("Long URL: {}", long),
                None => println!("Short URL not found: {}", args[2]),
            }
        }
        _ => println!("Unknown command: {}", args[1]),
    }

}
