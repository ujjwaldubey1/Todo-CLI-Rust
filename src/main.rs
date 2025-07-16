use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Debug)]
struct task {
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
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)?;
    let tasks = task {
        id: 1,
        title: data.to_string(),
        done: false,
    };
    writeln!(file, "{:?}", tasks)?;
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
