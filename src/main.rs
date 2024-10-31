use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{self, BufReader};
use std::path::{Path};
use std::env;
use chrono::Local;


const TODO_FILE: &str = "todo_list.json";

/*
1. I will add Telegram bot feature once I get the idea
to list all files/doc from the bot to get a list in the terminal.
If you have an idea/code that works, you can PR.

2. YOu can Automate the step's to add this directly to shell.
*/

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
    date: String,
}
// For loading task's
fn load_tasks() -> Vec<Task> {
    if Path::new(TODO_FILE).exists() {
        let file = File::open(TODO_FILE).expect("Error opening file.");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Error reading JSON")
    } else {
        vec![]
    }
}
// For saving it to json
fn save_tasks(tasks: &Vec<Task>) {
    let file = File::create(TODO_FILE).expect("Error creating file.");
    serde_json::to_writer_pretty(file, tasks).expect("Error writing JSON.");
}
// Fo r adding task's
fn add_task(description: String) {
    let mut tasks = load_tasks();
    let current_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    tasks.push(Task {
        description,
        completed: false,
        date: current_date,
    });
    save_tasks(&tasks);
    println!("Task successfully added.");
}
// Listing from file's
fn list_tasks() {
    let tasks = load_tasks();
    let header = "To-Do List";
    let line_length = 60;
    let centered_header = format!("{:^width$}", header, width = line_length);

    println!("\n{}", "=".repeat(line_length));
    println!("{}", centered_header);
    println!("{}", "=".repeat(line_length));
    
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        println!("{:<10} {:<10} {:<20} {}", "Task No", "Status", "Date", "Description");
        println!("{}", "=".repeat(line_length));
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[✓]" } else { "[ ]" };
            println!("{:<10} {:<10} {:<20} {}", i + 1, status, task.date, task.description);
        }
    }
    println!("{}", "=".repeat(line_length));
}
// For Marking's
fn complete_task(task_num: usize) {
    let mut tasks = load_tasks();
    if task_num > 0 && task_num <= tasks.len() {
        tasks[task_num - 1].completed = true;
        save_tasks(&tasks);
        println!("Task marked as completed.");
    } else {
        println!("Invalid task number.");
    }
}

fn uncomplete_task(task_num: usize) {
    let mut tasks = load_tasks();
    if task_num > 0 && task_num <= tasks.len() {
        tasks[task_num - 1].completed = false;
        save_tasks(&tasks);
        println!("Task marked as uncompleted.");
    } else {
        println!("Invalid task number.");
    }
}
// For removing
fn remove_task(task_num: usize) {
    let mut tasks = load_tasks();
    if task_num > 0 && task_num <= tasks.len() {
        tasks.remove(task_num - 1);
        save_tasks(&tasks);
        println!("Task removed.");
    } else {
        println!("Invalid task number.");
    }
}
// Clearing All
fn clear_all_tasks() {
    println!("Are you sure you want to clear all tasks? (y/n): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    if input.trim().eq_ignore_ascii_case("y") {
        save_tasks(&vec![]); // Save an empty list to clear all tasks
        println!("All tasks have been cleared.");
    } else {
        println!("Clear action canceled.");
    }
}
// Manual Usage 
fn print_usage() {
    println!("CLI To-Do with Telegram bot integration\n");
    println!("Usage: todo <COMMAND> [OPTIONS]\n");
    println!("Commands:");
    println!("  add <description>        Add a new task");
    println!("  list                     List all tasks");
    println!("  remove <task_num>        Remove a task");
    println!("  comp <task_num>          Mark a task as completed");
    println!("  uncomp <task_num>        Unmark a completed task");
    println!("  clr                      Clear all tasks");
    println!("  help                     Display this help message");
}

// Main Funciton's
fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
       
        "add" => {
            if args.len() < 3 {
                println!("Error: Missing description for new task.");
                print_usage();
            } else {
                add_task(args[2..].join(" "));
            }
        }
        "list" => list_tasks(),
        "remove" => {
            if args.len() < 3 {
                println!("Error: Missing task number.");
                print_usage();
            } else if let Ok(task_num) = args[2].parse() {
                remove_task(task_num);
            } else {
                println!("Error: Invalid task number.");
            }
        }
        "comp" => {
            if args.len() < 3 {
                println!("Error: Missing task number.");
                print_usage();
            } else if let Ok(task_num) = args[2].parse() {
                complete_task(task_num);
            } else {
                println!("Error: Invalid task number.");
            }
        }
        "uncomp" => {
            if args.len() < 3 {
                println!("Error: Missing task number.");
                print_usage();
            } else if let Ok(task_num) = args[2].parse() {
                uncomplete_task(task_num);
            } else {
                println!("Error: Invalid task number.");
            }
        }
         "clr" => clear_all_tasks(),
        "help" => print_usage(),
        _ => {
            println!("Error: Unknown command.");
            print_usage();
        }
    }
    
}
