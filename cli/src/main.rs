// add,remove,complete,list


use std::fs::{File,OpenOptions};
use serde::{Deserialize, Serialize};
use std::io::{self, BufReader, Write};
use std::path::Path;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use std::process;

//File work
const TODO_FILE: &str="todo_list.json";




struct Task{
    description: String,
    completed:bool,
}

// Loading from File
fn load_tasks(){}
fn save_taska(){}
fn add_tasks(){}
fn list_tasks(){}
fn complete_tasks(){}
fn remove_tasks(){}
fn send_2tele(){}

fn main(){}