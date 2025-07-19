Rust To-Do CLI Application
This is a simple command-line interface (CLI) application built with Rust to manage your to-do tasks. It allows you to add new tasks, view existing tasks, and mark tasks as done, with data persistence using a task.json file.

Features
Add Tasks: Easily add new tasks to your to-do list.

View Tasks: Display all your current tasks.

Mark Tasks Done: Mark specific tasks as completed using their ID.

Show Done Tasks: View only the tasks that have been marked as done.

Data Persistence: Tasks are saved to and loaded from a task.json file, so your data remains even after closing the application.

How to Run
To run this project, you will need to have Rust and Cargo (Rust's package manager) installed on your system. If you don't have them, you can install them by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install

Clone the repository (if applicable) or navigate to your project directory:

# If you have a Git repository:
git clone <your-repo-url>
cd <your-project-directory>

# If you are already in the project directory:
# (no action needed)

Build the project:

cargo build --release

This command compiles your project and creates an executable in the target/release/ directory. Using --release creates an optimized executable.

Run the application:

cargo run

Alternatively, you can run the compiled executable directly from the target/release/ directory:

On Windows:

.\target\release\todo_cli.exe

(Replace todo_cli.exe with your actual executable name if different, e.g., main.exe as seen in your file structure image)

On macOS/Linux:

./target/release/todo_cli

Usage
When you run the application, you will be prompted to "Enter commands". Here are the available commands:

Add input: Prompts you to enter the title of a new task.

Get tasks: Displays all tasks currently saved in task.json.

mark done: Prompts you to enter the ID of the task you want to mark as done.

show done tasks: Displays only the tasks that have been marked as done.

Example Workflow:

Enter commands
Add input
enter value
Buy groceries
Data is stored!

Enter commands
Add input
enter value
Finish Rust project
Data is stored!

Enter commands
Get tasks
Your tasks are:
[
  {
    "id": 1,
    "title": "Buy groceries",
    "done": false
  },
  {
    "id": 2,
    "title": "Finish Rust project",
    "done": false
  }
]

Enter commands
mark done
Enter the Id of task
1
Task with ID 1 marked as done.

Enter commands
show done tasks
--- Done Tasks ---
ID: 1, Title: Buy groceries
------------------

Enter commands
Get tasks
Your tasks are:
[
  {
    "id": 1,
    "title": "Buy groceries",
    "done": true
  },
  {
    "id": 2,
    "title": "Finish Rust project",
    "done": false
  }
]

Dependencies
This project uses the following external crate:

serde: For serialization and deserialization of Rust data structures to and from JSON.

serde = { version = "1.0", features = ["derive"] }

serde_json = "1.0"

These dependencies are specified in your Cargo.toml file.
