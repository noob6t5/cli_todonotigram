use std::fs::File; 
use serde::{Deserialize, Serialize};
use std::io::BufReader; 
use std::path::Path;
use clap::{Parser, Subcommand};
use chrono::Local; 


// File work
const TODO_FILE: &str = "todo_list.json";
// Removed CONFIG_FILE 

// #[derive(Serialize, Deserialize, Debug)]
// struct Config {
//     telegram_bot_token: String,
//     telegram_chat_id: String,
// }

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
    date: String, // New field for the date
}

// Loading Config
// fn load_config() -> Config {
//     let file = File::open("config.json").expect("Unable to open config file");
//     serde_json::from_reader(file).expect("Error reading config JSON")
// }

// Loading from File JSON files
fn load_tasks() -> Vec<Task> {
    if Path::new(TODO_FILE).exists() {
        let file = File::open(TODO_FILE).expect("Error while Opening File.");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Error while reading JSON")
    } else {
        vec![]
    }
}

// Saving to JSON Files
fn save_tasks(tasks: &Vec<Task>) {
    let file = File::create(TODO_FILE).expect("Error While Creating File..");
    serde_json::to_writer_pretty(file, tasks).expect("Error while writing JSON.");
}

// To Add new Task's
fn add_task(description: String) {
    let mut tasks = load_tasks();
    let current_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string(); // Get current date
    tasks.push(Task {
        description,
        completed: false,
        date: current_date, // Store the current date
    });
    save_tasks(&tasks);
    println!("Task Added ...");
}

// Listing task's to user's
fn list_tasks() {
    let tasks = load_tasks();
    println!("\n To-Do List:\n{}", "=".repeat(30));
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        println!("{:<10} {:<10} {:<20} {}", "Task No", "Status", "Date", "Description");
        println!("{}", "=".repeat(60));
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[✓]" } else { "[ ]" };
            println!("{:<10} {:<10} {:<20} {}", i + 1, status, task.date, task.description);
        }
    }
    println!("{}", "=".repeat(30));
}

// Completing & Marking
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

// Uncompleting tasks
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

// Removing tasks
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

// Bot Integration only for sending files, documents's......
// Sending a File to Telegram   fn send2_tele(){}


// CLI
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Simple CLI To-Do with Telegram bot integration")]
struct Cli {
    #[command(subcommand)]
    action: Actions,
}

#[derive(Subcommand)]
enum Actions {
    Add { description: String },
    List,
    Remove { task_num: usize },
    Complete { task_num: usize },
    Uncomplete { task_num: usize }, // New command for uncomplete
  /* Send {
        file_path: String,
        #[arg(long)]
        purpose: String,
    }, */
    Help, // Help command for usage
}

fn main() {
    let args = Cli::parse();

    match args.action {
        Actions::Add { description } => add_task(description),
        Actions::List => list_tasks(),
        Actions::Remove { task_num } => remove_task(task_num),
        Actions::Complete { task_num } => complete_task(task_num),
        Actions::Uncomplete { task_num } => uncomplete_task(task_num), // Handle uncomplete
      //  Actions::Send { file_path, purpose } => send_file_to_telegram(&file_path, &purpose),
        Actions::Help => {
            println!("Simple CLI To-Do with Telegram bot integration\n");
            println!("Usage: cli <COMMAND>\n");
            println!("Commands:");
            println!("  add       Add a new task");
            println!("  list      List all tasks");
            println!("  remove    Remove a task");
            println!("  complete   Mark a task as completed");
            println!("  uncomplete Mark a task as uncompleted");
           // println!("  send      Send a file to Telegram");
            println!("  help      Print this message");
            println!("\nOptions:");
            println!("  -h, --help  Print help");
        }
    }
}
