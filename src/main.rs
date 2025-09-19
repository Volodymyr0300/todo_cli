use std::{io::{self, Write}, path::Path, fs};

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

fn load_tasks(filename: &str) -> Result<Vec<Task>, io::Error> { // Function to load tasks from a file
    if !Path::new(filename).exists() { // Check if file exists
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(filename)?; // Read file content and transform it into a string
    let mut tasks: Vec<Task> = Vec::new(); // Initialize empty vector

    for (i, line) in content.lines().enumerate() {
        let mut parts = line.splitn(2, '|'); // Split into at most 2 parts
        let flag = parts.next().unwrap_or("0"); // The line tries to get the second part of the string (the part after the first |). Default to "0" if no flag
        let desc = parts.next().unwrap_or("").to_string(); // It takes the part of the line before the first |. Default to empty string if no description

        let done = flag == "1"; // Determine if task is done
        tasks.push(Task {
            id: i + 1,
            description: desc, // Use the description
            done, // Use the done boolean
        });
    }
    Ok(tasks) // Return the vector of tasks
}

fn save_tasks (filename: &str, tasks: &Vec<Task>) -> Result<(), io::Error> { // Function to save tasks to a file
    let mut lines: Vec<String> = Vec::new(); // Initialize empty vector to hold lines
    
    for task in tasks { // Iterate over each task
        let flag = if task.done {"1"} else {"0"}; // Convert boolean to "1" or "0"
        lines.push(format!("{}|{}", flag, task.description)); // Format each task as "flag|description"
    }

    let content = lines.join("\n"); // Join all lines with newline characters
    fs::write(filename, content)?; // Write the content to the file if successful
    
    Ok(()) // Return Ok if successful
}

fn add_task(tasks: &mut Vec<Task>, description: String) { // Function to add a new task
    let id = tasks.len() + 1; // New task ID is the next number in sequence
    let t = Task { // Create a new task
        id,
        description,
        done: false,
    };
    tasks.push(t); // Add the new task to the vector
}

fn list_tasks(tasks: &Vec<Task>) { // Function to list all tasks
    if tasks.is_empty() { // Check if there are no tasks
        println!("No tasks found."); 
        return;
    }
    println!("ID Done Description");
    for task in tasks { // Iterate over each task
        let status = if task.done {"x"} else {" "}; // Mark done tasks with 'x' and not done with space
        println!("{:>2} [{}] {}", task.id, status, task.description); // Print task details
    }
}

fn complete_task(tasks: &mut Vec<Task>, id: usize) -> bool { // Function to mark a task as complete by ID
    for task in tasks.iter_mut() { // Iterate over each task mutably 
        if task.id == id { // Check if the task ID matches the given ID
            task.done = true;
            return true;
        }
    }
    false
}

fn remove_task(tasks: &mut Vec<Task>, id: usize) -> bool { // Function to remove a task by ID
    if id == 0 || id > tasks.len() {return false;}; // Validate ID range
    tasks.remove(id - 1); // Remove the task (adjust for 0-based index)
    for (i, task) in tasks.iter_mut().enumerate() { // Reassign IDs to maintain sequence after removal
        task.id = i + 1; // IDs start from 1
    }
    true
}

fn print_help() { // Function to print help message
    println!("Commands:");
    println!("  add <description>   - add a new task");
    println!("  list                - list tasks");
    println!("  done <id>           - mark a task as done");
    println!("  remove <id>         - remove a task");
    println!("  save                - save tasks to file");
    println!("  quit                - save and exit");
    println!("  help                - show this help");
}

fn main() {
    
}
