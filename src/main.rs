use std::io;
fn main() {
    println!("Please write somthing !");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error in reading input");
    print!("{}", input);
    print!("{}", input);
}
