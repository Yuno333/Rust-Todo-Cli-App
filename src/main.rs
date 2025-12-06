// Import standard library modules for file operations, I/O, and command-line arguments
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::env;

// Define the Todo struct to represent a single task
// #[derive(Debug, Clone)] automatically implements Debug (for printing) and Clone (for copying)
#[derive(Debug, Clone)]
struct Todo {
    id: usize,           // Unique identifier for each task
    task: String,        // The actual task description
    completed: bool,     // Status: true if done, false if pending
}

// Implementation block for Todo struct - defines methods that operate on Todo instances
impl Todo {
    // Constructor: creates a new Todo with the given id and task
    // Returns Self (which is Todo) with completed set to false by default
    fn new(id: usize, task: String) -> Self {
        Todo {
            id,
            task,
            completed: false,
        }
    }

    // Deserializes a Todo from a line of text (format: "id|task|completed")
    // Returns Option<Self>: Some(Todo) if parsing succeeds, None if it fails
    fn from_line(line: &str) -> Option<Self> {
        // Split the line by pipe character into parts
        let parts: Vec<&str> = line.split('|').collect();
        
        // Validate that we have exactly 3 parts
        if parts.len() != 3 {
            return None;
        }
        
        // Parse and construct Todo, using ? operator to propagate parse errors
        Some(Todo {
            id: parts[0].parse().ok()?,      // Parse string to usize, return None if fails
            task: parts[1].to_string(),      // Convert &str to owned String
            completed: parts[2] == "true",   // Check if completion string equals "true"
        })
    }

    // Serializes a Todo to a string for file storage
    // &self means this borrows the Todo (doesn't take ownership)
    fn to_line(&self) -> String {
        format!("{}|{}|{}", self.id, self.task, self.completed)
    }
}

// TodoList manages the collection of todos and handles persistence
struct TodoList {
    todos: Vec<Todo>,      // Vector (dynamic array) holding all todos
    filename: String,      // Name of the file where todos are stored
}

// Implementation block for TodoList
impl TodoList {
    // Constructor: creates a new TodoList and loads existing todos from file
    fn new(filename: &str) -> Self {
        let mut list = TodoList {
            todos: Vec::new(),               // Initialize empty vector
            filename: filename.to_string(),   // Convert &str to owned String
        };
        list.load();  // Load todos from file immediately
        list
    }

    // Loads todos from the file into memory
    // &mut self means this method can modify the TodoList
    fn load(&mut self) {
        // Try to open the file; if it doesn't exist or fails, silently continue
        if let Ok(file) = File::open(&self.filename) {
            let reader = BufReader::new(file);  // Wrap file in buffered reader for efficiency
            
            // Iterate through each line in the file
            // flatten() removes any Err results from the iterator
            for line in reader.lines().flatten() {
                // Try to parse the line into a Todo
                if let Some(todo) = Todo::from_line(&line) {
                    self.todos.push(todo);  // Add successfully parsed todo to vector
                }
            }
        }
    }

    // Saves all todos to the file
    // Returns io::Result<()> to indicate success or failure
    fn save(&self) -> io::Result<()> {
        // Open file with these options:
        // - write(true): allow writing
        // - create(true): create file if it doesn't exist
        // - truncate(true): clear existing content before writing
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?;  // ? operator propagates errors up
        
        // Write each todo as a line to the file
        for todo in &self.todos {
            writeln!(file, "{}", todo.to_line())?;  // ? propagates write errors
        }
        Ok(())  // Return success
    }

    // Adds a new todo with the given task description
    fn add(&mut self, task: String) {
        // Find the highest existing ID and add 1, or use 1 if no todos exist
        let id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        
        self.todos.push(Todo::new(id, task));  // Add new todo to vector
        self.save().ok();  // Save to file, ignore errors with .ok()
        println!("✓ Added task #{}", id);
    }

    // Lists all todos with formatted output
    fn list(&self) {
        // Check if there are no todos
        if self.todos.is_empty() {
            println!("No tasks yet! Add one with: todo add <task>");
            return;
        }

        println!("\n📋 Your Tasks:");
        println!("{}", "─".repeat(50));  // Print a horizontal line
        
        // Iterate through all todos
        for todo in &self.todos {
            // Choose checkmark or empty space based on completion status
            let status = if todo.completed { "✓" } else { " " };
            
            // Apply strikethrough formatting to completed tasks using ANSI escape codes
            // \x1b[9m starts strikethrough, \x1b[0m resets formatting
            let task = if todo.completed {
                format!("\x1b[9m{}\x1b[0m", todo.task)
            } else {
                todo.task.clone()  // Clone creates a copy of the String
            };
            
            println!("[{}] #{}: {}", status, todo.id, task);
        }
        println!("{}", "─".repeat(50));
    }

    // Marks a todo as completed by its ID
    fn complete(&mut self, id: usize) {
        // Find the todo with matching ID using iter_mut (mutable iterator)
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;  // Mark as completed
            self.save().ok();       // Save changes to file
            println!("✓ Completed task #{}", id);
        } else {
            println!("✗ Task #{} not found", id);
        }
    }

    // Deletes a todo by its ID
    fn delete(&mut self, id: usize) {
        // Find the position (index) of the todo with matching ID
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(pos);  // Remove todo at that position
            self.save().ok();        // Save changes to file
            println!("✓ Deleted task #{}", id);
        } else {
            println!("✗ Task #{} not found", id);
        }
    }

    // Removes all completed todos from the list
    fn clear_completed(&mut self) {
        let before = self.todos.len();  // Count todos before clearing
        
        // retain() keeps only todos that match the condition (not completed)
        self.todos.retain(|t| !t.completed);
        
        let removed = before - self.todos.len();  // Calculate how many were removed
        self.save().ok();
        println!("✓ Removed {} completed task(s)", removed);
    }
}

// Prints usage instructions for the CLI
fn print_help() {
    println!("\n📝 Rust Todo CLI");
    println!("\nUsage:");
    println!("  todo add <task>        Add a new task");
    println!("  todo list              List all tasks");
    println!("  todo done <id>         Mark task as complete");
    println!("  todo delete <id>       Delete a task");
    println!("  todo clear             Remove all completed tasks");
    println!("  todo help              Show this help message\n");
}

// Main entry point of the program
fn main() {
    // Collect command-line arguments into a vector
    // args[0] is the program name, args[1] is the command, etc.
    let args: Vec<String> = env::args().collect();
    
    // Create a new TodoList instance, loads todos from "todos.txt"
    let mut todo_list = TodoList::new("todos.txt");

    // Check if user provided at least one argument (the command)
    if args.len() < 2 {
        print_help();
        return;  // Exit early if no command given
    }

    // Match on the command (first argument after program name)
    // as_str() converts String to &str for pattern matching
    match args[1].as_str() {
        "add" => {
            // Ensure user provided a task description
            if args.len() < 3 {
                println!("✗ Please provide a task description");
                return;
            }
            
            // Join all arguments after "add" into a single task string
            // This allows tasks with spaces like: todo add Buy milk and eggs
            let task = args[2..].join(" ");
            todo_list.add(task);
        }
        
        "list" => {
            todo_list.list();
        }
        
        "done" => {
            if args.len() < 3 {
                println!("✗ Please provide a task ID");
                return;
            }
            
            // Try to parse the ID string into a number
            if let Ok(id) = args[2].parse::<usize>() {
                todo_list.complete(id);
            } else {
                println!("✗ Invalid task ID");
            }
        }
        
        "delete" => {
            if args.len() < 3 {
                println!("✗ Please provide a task ID");
                return;
            }
            
            // Parse ID and delete the todo
            if let Ok(id) = args[2].parse::<usize>() {
                todo_list.delete(id);
            } else {
                println!("✗ Invalid task ID");
            }
        }
        
        "clear" => {
            todo_list.clear_completed();
        }
        
        "help" => {
            print_help();
        }
        
        // Default case: unknown command
        _ => {
            println!("✗ Unknown command: {}", args[1]);
            print_help();
        }
    }
}