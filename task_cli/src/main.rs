use std::env;
use std::fs;
use serde::{Serialize, Deserialize};

const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
    done: bool,
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let content = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write(FILE_PATH, content).expect("Failed to write tasks to file");
}
fn add_taks(title: String) -> Task {
    Task {
        title,
        done: false,
    }
}

fn remove_task(tasks: &mut Vec<Task>, title: &str) {
    tasks.retain(|task| task.title != title);
}

fn  mark_done(task: &mut Task) {
    task.done = true;
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!("{} - {}", task.title, if task.done { "Done" } else { "Not Done" });
    }
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <command> [arguments]");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: todo add <task_title>");
                return;
            }
            let task = add_taks(args[2].clone());
            tasks.push(task);
            save_tasks(&tasks);
            println!("Task added: {}", args[2]);
        }
        "remove" => {
            if args.len() < 3 {
                println!("Usage: todo remove <task_title>");
                return;
            }
            remove_task(&mut tasks, &args[2]);
            save_tasks(&tasks);
            println!("Task removed: {}", args[2]);
        }
        "done" => {
            if args.len() < 3 {
                println!("Usage: todo done <task_title>");
                return;
            }
            if let Some(task) = tasks.iter_mut().find(|t| t.title == args[2]) {
                mark_done(task);
                save_tasks(&tasks);
                println!("Task marked as done: {}", args[2]);
            } else {
                println!("Task not found: {}", args[2]);
            }
        }
        "list" => {
            list_tasks(&tasks);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }       
}
