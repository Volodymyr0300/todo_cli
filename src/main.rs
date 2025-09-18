use std::{io::{self, Write}, path::Path, fs};

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

fn load_tasks(filename: &str) -> Result<Vec<Task>, io::Error> {
    if !Path::new(filename).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(filename)?;
    let mut tasks: Vec<Task> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(3, ',').collect();
        if parts.len() == 3 {
            let id = parts[0].parse::<usize>().unwrap_or(0);
            let description = parts[1].to_string();
            let done = parts[2] == "1";
            tasks.push(Task { id, description, done });
        }
    }
    Ok(tasks)
}
    

fn main() {
    println!("Hello, world!");
}
