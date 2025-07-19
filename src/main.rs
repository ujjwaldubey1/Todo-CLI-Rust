use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]

struct Task {
    id: i32,
    title: String,
    done: bool,
}
fn main() -> std::io::Result<()> {
    println!("Enter commands");
    let mut _command_input = String::new();
    io::stdin()
        .read_line(&mut _command_input)
        .expect("Error in data entry");

    let file_path = String::from("task.json");

    if _command_input.trim() == "Add input" {
        let user_todo_input = take_input();
        save_to_file(&user_todo_input, &file_path);
        print!("Data is stored!");
    } else if _command_input.trim() == "Get tasks" {
        let taks = read_todo_file(&file_path);
        println!("Your tasks are \n {}", taks);
    } else if _command_input.trim() == "mark done" {
        println!("Enter the Id of task");
        let mut id_input = String::new();
        io::stdin().read_line(&mut id_input).expect("Not valid id");

        let id: i32 = match id_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("invalid number");
                return Ok(());
            }
        };

        mark_done(id, &file_path);
    } else if _command_input.trim() == "done tasks" {
        show_done_tasks(&file_path);
    }

    Ok(())
}

// fn take_input() -> String {
//     println!("Please write somthing !");
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Error in reading input");
//     input.trim().to_string()
// }

fn take_input() -> String {
    println!("enter value");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error occured");
    input.trim().to_string()
}

fn save_to_file(data: &str, file_name: &str) -> std::io::Result<()> {
    // let mut file = OpenOptions::new()
    //     .append(true)
    //     .create(true)
    //     .open(file_name)?;
    let mut tasks: Vec<Task> = if let Ok(content) = fs::read_to_string(file_name) {
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    tasks.push(Task {
        id: new_id,
        title: data.to_string(),
        done: false,
    });
    let json = serde_json::to_string_pretty(&tasks).expect("serialization faild");
    fs::write(file_name, json)?;

    Ok(())
}

// fn save_to_file(data: &str, file_name: &str) -> std::io::Result<()> {
//     let mut file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open(file_name)?;
//     writeln!(file, "{}", data)?;
//     Ok(())
// }

fn read_todo_file(file: &str) -> String {
    let todos_from_file = fs::read_to_string(file).expect("Error in reading");
    todos_from_file
}

fn mark_done(id: i32, file: &str) -> std::io::Result<()> {
    let mut tasks: Vec<Task> = if let Ok(content) = fs::read_to_string(file) {
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
    }
    let json = serde_json::to_string_pretty(&tasks)?;
    fs::write(file, json)?;
    Ok(())
}

fn show_done_tasks(file_path: &str) -> std::io::Result<()> {
    let tasks: Vec<Task> = if let Ok(content) = fs::read_to_string(file_path) {
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    let mut find_done_task = false;

    for task in tasks {
        if task.done {
            print!("id {}, title {}", task.id, task.title);
            find_done_task = true;
        }
    }

    if !find_done_task {
        print!("Taks did't found");
    }
    

    // let tasks: Vec<Task> = if let Ok(content) = fs::read_to_string(file_path) {
    //     serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    // } else {
    //     vec![]
    // };

    // let mut found_done_task = false;
    // for task in tasks {
    //     if task.done {
    //         print!("ID:{} , Title {}", task.id, task.title);
    //         found_done_task = true;
    //     }
    // }

    // if !found_done_task {
    //     print!("No done task found");
    // }

    Ok(())
}
