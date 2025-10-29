use std::process::{self};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self};
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum TodoError {
    Io(io::Error),
    Parse(ParseIntError),
    NotFound,
    Serialization(serde_json::Error),
    InvalidInput,
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::Io(err) => write!(f, "I/O Error: {}", err),
            TodoError::Parse(err) => write!(f, "Parsing Error: Expected a number, got something else. ({})", err),
            TodoError::NotFound => write!(f, "Application Error: Task not found at that index."),
            TodoError::Serialization(err) => write!(f, "Data Error: Could not read or write data file. ({})", err),
            TodoError::InvalidInput => write!(f, "Input Error: Please enter a valid command or value."),
        }
    }
}

impl From<io::Error> for TodoError {
    fn from(err: io::Error) -> Self {
        TodoError::Io(err)
    }
}

impl From<ParseIntError> for TodoError {
    fn from(err: ParseIntError) -> Self {
        TodoError::Parse(err)
    }
}

impl From<serde_json::Error> for TodoError {
    fn from(err: serde_json::Error) -> Self {
        TodoError::Serialization(err)
    }
}

pub type Result<T> = std::result::Result<T, TodoError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    desc : String,
    progress : TaskStatus,
    prio : i32,
}

#[derive(Serialize, Deserialize, Debug)]
enum TaskStatus {
    Done,
    InProgress,
    Pending,
}

fn main() {
    let mut tasks = load_tasks();
    
    loop {
        println!("\nWhat do you want to do?\nS: Show Task list\nC: Create a new task\nD: Delete a task\nU: Update a task\nE: Exit");
        
        let command = match read_line_input() {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Error reading command: {}", e);
                continue;
            }
        };
        
        match command.as_str() {
            "c" => create_task(&mut tasks),
            "d" => delete_task(&mut tasks),
            "u" => update_task(&mut tasks),
            "s" => print_tasklist(&mut tasks),
            "e" => {
                let _ = save_tasks(&tasks);
                exit();
            },
            _ => println!("Unknown command, try again."),
        };
        
        
        match save_tasks(&tasks) {
            Ok(()) => println!("Saved list!"),
            Err(e) => eprintln!("-> Warning: Could not save to file! {}", e),
        }
    }
}

fn load_tasks() -> Vec<Task> {
    let path = "tasks.json";
    match try_load_tasks(path) {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Error loading tasks: {}. Starting with an empty list.", e);
            Vec::new()
        }
    }
}

fn try_load_tasks(path: &str) -> Result<Vec<Task>> {
    let data = fs::read_to_string(path)?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write("tasks.json", json)?;
    Ok(())
}

fn create_task(tasks: &mut Vec<Task>) {
    println!("Creating a new task. What's the description?");
    let task_desc = match read_line_input() {
        Ok(s) => s,
        Err(_) => { 
            eprintln!("Failed to read description.");
            return;
        }
    };
    
    let task_prio = loop {
        println!("And what's the priority? (Enter an integer)");
        let input = match read_line_input() {
            Ok(s) => s,
            Err(_) => continue, // io error
        };
        
        match input.parse::<i32>() {
            Ok(p) => break p,
            Err(e) => {
                eprintln!("{}", TodoError::Parse(e));
                continue; // invalid
            }
        }
    };

    let new_task = Task  {
        desc: task_desc,
        progress: TaskStatus::Pending,
        prio: task_prio,
    };
    tasks.push(new_task);
    println!("Task added!");
}

fn delete_task(tasks: &mut Vec<Task>) {   
    println!("Deleting a task."); 
    print_tasklist(tasks);

    // get index of task
    let task_to_delete = loop {
        println!("What task would you like to remove? (Enter an integer)");
        let input = match read_line_input() {
            Ok(s) => s,
            Err(_) => continue, // io error
        };
        
        match input.parse::<usize>() {
            Ok(p) => break p,
            Err(e) => {
                eprintln!("{}", TodoError::Parse(e));
                continue; // invalid
            }
        }
    };

    if tasks.len() >= task_to_delete {
        tasks.remove(task_to_delete);
        println!("Task removed!")
    }
    else {
        println!("Could not find task!")
    }
}

fn update_task(tasks: &mut Vec<Task>) {
    println!("Updating a task.");
    print_tasklist(tasks);

    // get index of task
    let task_to_update = loop {
        println!("What task would you like to update? (Enter an integer)");
        let input = match read_line_input() {
            Ok(s) => s,
            Err(_) => continue, // io error
        };
        
        match input.parse::<usize>() {
            Ok(p) => break p,
            Err(e) => {
                eprintln!("{}", TodoError::Parse(e));
                continue; // invalid
            }
        }
    };
    
    if tasks.len() >= task_to_update {
        println!("What progress would you like to set to {}?\nD : Done\nI : In Progress\nP : Pending", tasks[task_to_update].desc);
        let update = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_lowercase()
        };
        match update.as_str() {
            "d" => tasks[task_to_update].progress = TaskStatus::Done,
            "i" => tasks[task_to_update].progress = TaskStatus::InProgress,
            "p" => tasks[task_to_update].progress = TaskStatus::Pending,
            _ => println!("Unknown command, try again."),
        }
        
        let new_prio = loop {
            println!("What's the new priority of {}?", tasks[task_to_update].desc);
            let input = match read_line_input() {
                Ok(s) => s,
                Err(_) => continue, // io error
        };
        
        match input.parse::<i32>() {
            Ok(p) => break p,
            Err(e) => {
                eprintln!("{}", TodoError::Parse(e));
                continue; // invalid
            }
        }
    };
        tasks[task_to_update].prio = new_prio;
        println!("Task updated!")
    }
    else {
        println!("Could not find task!")
    }
}

fn print_tasklist(tasks: &mut Vec<Task>) {
    if tasks.len() > 0 {
        for i in 1..=tasks.len() {
            println!("{}. Task: {}, Progress: {:?}, Priority: {}", 
            i-1, tasks[i-1].desc, tasks[i-1].progress, tasks[i-1].prio)
        }
    }
    else {
        println!("Task list is empty!");
    }
}

fn read_line_input() -> Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_owned())
}

fn exit() {
    println!("Exiting application!");
     process::exit(1);
}