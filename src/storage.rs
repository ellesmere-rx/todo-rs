use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub struct Task {
    pub text: String,
    pub done: bool,
}

impl Task {
    fn new(done: bool, text: String) -> Self {
        Task { done, text }
    }
}

pub fn create_and_write_savefile(path: &Path, tasks: &Vec<Task>) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    for task in tasks {
        writeln!(file, "{} {}", task.done, task.text)?;
    }

    Ok(())
}

pub fn read_savefile(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    Ok(lines)
}

pub fn parse_lines_to_tasks(lines: Vec<String>) -> Option<Vec<Task>> {
    let mut tasks = Vec::new();

    for line in lines {
        match line.split_once(' ') {
            Some((done_str, text)) => match done_str.parse::<bool>() {
                Ok(done) => {
                    tasks.push(Task::new(done, text.to_string()));
                }
                Err(_) => return None,
            },
            None => return None,
        }
    }
    Some(tasks)
}
