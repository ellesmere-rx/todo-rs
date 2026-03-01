use crate::{
    fmt::{ask_confirmation, clear_screen, parse_task_index, print_help, print_usage},
    storage::Task,
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
                } else {
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
                    println!("Task #{} marked as {}.", idx + 1, status);
                } else {
                    println!("Invalid index! Use a number from 1 to {}.", tasks.len());
                    print_usage("mark");
                }
            }
            "rm" => {
                if args.is_empty() {
                    println!("{} tasks will be deleted, are you sure?", tasks.len());
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
            "save" => {
                println!("WIP");
            }
            "load" => {
                println!("WIP");
            }
            "help" => {
                print_help();
            }
            "cls" => {
                clear_screen();
            }
            "exit" => {
                println!("Bye!");
                break;
            }
            _ => {
                print_help();
            }
        }
    }
}
