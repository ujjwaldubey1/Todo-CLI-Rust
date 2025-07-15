use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    let user_input = take_input();
    save_to_file(&user_input, "task.txt");
    print!("Data is stored!");
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
    writeln!(file, "{}", data)?;
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
