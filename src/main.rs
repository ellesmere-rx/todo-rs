use crate::{
    fmt::{ask_confirmation, clear_screen, parse_path, parse_task_index, print_help, print_usage},
    storage::{Task, create_and_write_savefile, parse_lines_to_tasks, read_savefile},
};
mod fmt;
mod storage;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    fmt::print_menu();
    loop {
        let (command, args) = fmt::read_prompt();
        match command.as_str() {
            "add" => {
                if !args.is_empty() {
                    let task = Task {
                        text: args,
                        done: false,
                    };
                    tasks.push(task);
                    println!("Task {} added.", tasks.len());
                } else {
                    println!("Missing <task>!");
                    print_usage("add");
                }
            }
            "ls" => {
                if args.is_empty() {
                    if tasks.is_empty() {
                        println!("No tasks yet.");
                    } else {
                        for (i, task) in tasks.iter().enumerate() {
                            let idx = i + 1;
                            let status = if task.done { "[x]" } else { "[ ]" };
                            println!("  {}. {} {}", idx, status, task.text);
                        }
                    }
                } else {
                    if let Some(idx) = parse_task_index(&args, tasks.len()) {
                        let task = &tasks[idx];
                        let status = if task.done { "[x]" } else { "[ ]" };
                        println!("  {}. {} {}", idx + 1, status, task.text);
                    } else {
                        println!("Invalid index! Use a number from 1 to {}.", tasks.len());
                        print_usage("ls");
                    }
                }
            }

            "mark" => {
                if args.is_empty() {
                    println!("Missing index!");
                    print_usage("mark");
                } else if let Some(idx) = parse_task_index(&args, tasks.len()) {
                    tasks[idx].done = !tasks[idx].done;
                    let status = if tasks[idx].done { "done" } else { "not done" };
                    println!("Task {} marked as {}.", idx + 1, status);
                } else {
                    println!("Invalid index! Use a number from 1 to {}.", tasks.len());
                    print_usage("mark");
                }
            }
            "rm" => {
                if args.is_empty() {
                    println!("{} tasks will be deleted, are you sure? [y/N]", tasks.len());
                    if ask_confirmation() {
                        tasks.clear();
                        println!("Tasks deleted.");
                    } else {
                        println!("Operation aborted.");
                        continue;
                    }
                } else {
                    if let Some(idx) = parse_task_index(&args, tasks.len()) {
                        tasks.remove(idx);
                        println!("Task {} deleted.", idx + 1);
                    } else {
                        println!("Invalid index! Use a number from 1 to {}.", tasks.len());
                        print_usage("rm");
                    }
                }
            }
            "clean" => {
                if !args.is_empty() {
                    println!("Too many arguments!");
                    continue;
                }
                let old_len = tasks.len();
                tasks.retain(|task| !task.done);
                println!("{} tasks cleaned.", old_len - tasks.len());
            }
            "save" => {
                if args.is_empty() {
                    println!("Missing path!");
                    print_usage("save");
                    continue;
                }
                let filepath = parse_path(args);
                match create_and_write_savefile(&filepath, &tasks) {
                    Ok(_) => {
                        println!(
                            "{} tasks saved to {} successfully.",
                            tasks.len(),
                            filepath.display()
                        )
                    }
                    Err(e) => {
                        println!("Error while saving file: {}", e)
                    }
                }
            }
            "load" => {
                if args.is_empty() {
                    println!("Missing path!");
                    print_usage("load");
                    continue;
                }
                let filepath = parse_path(args);
                match read_savefile(&filepath) {
                    Ok(lines) => {
                        let parsed_tasks = parse_lines_to_tasks(lines);
                        match parsed_tasks {
                            Some(loaded_tasks) => {
                                if loaded_tasks.len() == 0 {
                                    println!("Tasks file is empty, are you sure to load it? [y/N]");
                                    if ask_confirmation() {
                                        tasks = loaded_tasks;
                                        println!("{} loaded successfully.", tasks.len());
                                    } else {
                                        continue;
                                    }
                                } else {
                                    tasks = loaded_tasks;
                                    println!("{} loaded successfully.", tasks.len());
                                }
                            }
                            None => {
                                println!(
                                    "Error while parsing tasks. Most likely, file is corrupted."
                                );
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error while reading file: {}.", e);
                    }
                }
            }
            "help" => {
                print_help();
            }
            "cls" => {
                if !args.is_empty() {
                    println!("Too many arguments!");
                    continue;
                }
                clear_screen();
            }
            "exit" => {
                if !args.is_empty() {
                    println!("Too many arguments!");
                    continue;
                }
                println!("Bye!");
                break;
            }
            _ => {
                print_help();
            }
        }
    }
}
