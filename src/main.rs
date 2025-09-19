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
    

fn main() {
    
}
