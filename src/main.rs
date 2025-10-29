use std::process::{self};
pub struct Task {
    desc : String,
    progress : TaskStatus,
    prio : i32,
}

#[derive(Debug)]
enum TaskStatus {
    Done,
    InProgress,
    Pending,
}

fn main() {
    let mut tasks = Vec::<Task>::new();
    
    loop {
        println!("What do you want to do?\nS : Show the Task list\nC : Create a new task\nD : Delete a task\nU : Update a task\nE : Exit");
        let answer = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_lowercase()
        };
        
        match answer.as_str() {
            "c" => create_task(&mut tasks),
            "d" => delete_task(&mut tasks),
            "u" => update_task(&mut tasks),
            "s" => print_whole_tasklist(&mut tasks),
            "e" => exit(),
            _ => println!("Unknown command, try again."),
        }
    }
}

fn create_task(tasks: &mut Vec<Task>) {
    println!("Creating a new task. What's the description?");
    let task_desc = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().to_lowercase()
    };
    println!("And what's the priority?");
    let task_prio = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().to_lowercase()
    };

    let new_task = Task  {
        desc: task_desc,
        progress: TaskStatus::Pending,
        prio: task_prio.parse::<i32>().unwrap(),
    };
    tasks.push(new_task);
    println!("Task added!");
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("Deleting a task. Write the number of the task to be deleted.");
    print_whole_tasklist(tasks);

    // get index of task
    let task_to_delete = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().to_lowercase()
    };
    let index = task_to_delete.parse::<usize>().unwrap();
    if tasks.len() >= index { // if fits in span of vector, remove
        tasks.remove(index);
        println!("Task removed!")
    }
    else {
        println!("Could not find task!")
    }
}

fn update_task(tasks: &mut Vec<Task>) {
    println!("Updating a task. What task should be updated?");
    print_whole_tasklist(tasks);

    // get index of task
    let task_to_update = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().to_lowercase()
    };
    let index = task_to_update.parse::<usize>().unwrap();
    if tasks.len() >= index {
        println!("What progress would you like to set to {}?\nD : Done\nI : In Progress\nP : Pending", tasks[index].desc);
        let update = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_lowercase()
        };
        match update.as_str() {
            "d" => tasks[index].progress = TaskStatus::Done,
            "i" => tasks[index].progress = TaskStatus::InProgress,
            "p" => tasks[index].progress = TaskStatus::Pending,
            _ => println!("Unknown command, try again."),
        }
        println!("What's the new priority of {}?", tasks[index].desc);
        let prio = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_lowercase()
        };
        let new_prio = prio.parse::<i32>().unwrap();
        tasks[index].prio = new_prio;
        println!("Task updated!")
    }
    else {
        println!("Could not find task!")
    }
}

fn print_whole_tasklist(tasks: &mut Vec<Task>) {
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

fn exit() {
    println!("Exiting application!");
     process::exit(1);
}