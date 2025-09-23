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

fn print_help() { // Function to print help information
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
    let filename = "tasks.txt"; // File to store tasks

    let mut tasks = match load_tasks(filename) { // Load existing tasks from file
        Ok(t) => t, // If successful, use the loaded tasks
        Err(e) => { // If there's an error, print it and start with an empty list
            eprintln!("Failed to load tasks: {}", e);
            return;
        }
    };
    println!("{}", filename);
    println!("The vector is: {:?}", tasks);
    println!("Welcome to todo_cli! Type 'help' for commands.");

    loop {
        println!("> "); // Prompt for user input
        io::stdout().flush().expect("flush failed"); // Ensure prompt is displayed immediately
        // Without flush(), the user might type their response before the prompt even appears on the screen.

        let mut input = String::new(); // Initialize empty string for user input
        if io::stdin().read_line(&mut input).is_err() { // Read user input and handle potential errors
            println!("Error reading input. Try again."); // Handle read error gracefully
            continue;
        }

        let input = input.trim(); // Trim whitespace from input ends
        if input.is_empty() { // If input is empty, prompt again
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect(); // Split input into parts based on whitespace and collect into a vector
        let command = parts[0].to_lowercase(); // Get the command and convert to lowercase for case-insensitivity

        if command == "help" { // Show help information if user types 'help'
            print_help();
        } else if command == "add" { // Add a new task if user types 'add <description>'
            if parts.len() < 2 { // Ensure there's a description provided
                println!("Usage: add <description>"); // Show usage if description is missing
                continue; // Skip to next iteration of the loop
            }
            let description = parts[1..].join(" "); // Join all parts after the command to form the full description
            add_task(&mut tasks, description); // Add the new task to the list mutably
            println!("Task added.");
        } else if command == "list" { // List all tasks if user types 'list'
            list_tasks(&tasks); // List tasks (no mutation needed)
        } else if command == "done" { // Mark a task as done if user types 'done <id>'
            if parts.len() !=2 { // Ensure exactly one ID is provided
                println!("Usage: done <id>"); // Show usage if ID is missing or too many arguments
                continue;
            }
            match parts[1].parse::<usize>() { // Parse the ID from string to usize 
                Ok(id) => { //  If parsing is successful
                    if complete_task(&mut tasks, id) { // Mark the task as done mutably 
                        println!("Task {} marked as done.", id);
                    } else {
                        println!("No task with id {}.", id);
                    }
                }
                Err(_) => println!("Invalid id. Use a number."), 
            }
        } else if command == "remove" { // Remove a task if user types 'remove <id>'
            if parts.len() != 2 { // Ensure exactly one ID is provided
                println!("Usage: remove <id>"); // Show usage if ID is missing or too many arguments
                continue;
            }
            match parts[1].parse::<usize>() { //    Parse the ID from string to usize 
                Ok(id) => { //  If parsing is successful 
                    if remove_task(&mut tasks, id) { // Remove the task mutably 
                        println!("Task {} removed.", id);
                    } else {
                        println!("No task with id {}.", id);
                    }
                }
                Err(_) => println!("Invalid id. Use a number."),
            }
        } else if command == "save" { // Save tasks to file if user types 'save'
            match save_tasks(filename, &tasks) { // Save tasks (no mutation needed)
                Ok(()) => println!("Saved to {}.", filename), // If successful, confirm save 
                Err(e) => println!("Failed to save: {}. Exiting anyway.", e), // If there's an error, print it
            }
        } else if command == "quit" { // Save tasks and exit if user types 'quit'
            match save_tasks(filename, &tasks) { // Save tasks (no mutation needed)
                Ok(()) => println!("Saved. Goodbye!"), // If successful, confirm save and exit 
                Err(e) => println!("Failed to save: {}. Exiting anyway.", e), // If there's an error, print it
            }
            break; // Exit the loop and end the program
        } else {
            println!("Unknown command '{}'. Type 'help' for a list of commands.", command); // Handle unknown commands gracefully
        } 
    }
}