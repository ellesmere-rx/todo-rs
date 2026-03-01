use std::io::{self, Write};

pub fn print_menu() {
    println!(" = = = = = N O T O = = = = =");
    println!("- add <task>");
    println!("- ls [index]");
    println!("- mark <index>");
    println!("- rm <index>");
    println!("- save <path>");
    println!("- load <path>");
    println!("- help");
}

pub fn print_help() {
    let help = r#"
noto - simple to-do manager in rust

commands:
    add     <task>     Add a new task
    ls      [index]    Show all tasks or task by index
    mark    <index>    Mark task as done
    rm      [index]    Delete tasks or by index
    save    <path>     Save tasks to file
    load    <path>     Load tasks from file
    cls                Clear terminal
    exit               Exit noto
    help               Print help
    "#;
    println!("{}", help.trim());
}

pub fn read_prompt() -> (String, String) {
    print!("> ");
    io::stdout().flush().expect("can't flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error!");

    let trimmed = input.trim();

    let Some((cmd, rest)) = trimmed.split_once(' ') else {
        return (trimmed.to_string(), String::new());
    };

    (cmd.to_string(), rest.trim_start().to_string())
}

pub fn print_usage(command: &str) {
    println!("Invalid syntax!\n\nUsage:");

    match command {
        "add" => println!("    add <task>"),
        "ls" => println!("    ls [index]"),
        "mark" => println!("    mark <index>"),
        "rm" => println!("    rm [index]"),
        "save" => println!("    save [path]"),
        "load" => println!("    load <path>"),
        _ => {
            println!("  Unknown command: {}", command);
            println!("  Try: help");
            return;
        }
    }

    println!();
}

pub fn parse_task_index(s: &str, tasks_len: usize) -> Option<usize> {
    let num = match s.trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => return None,
    };

    if num == 0 || num > tasks_len {
        None
    } else {
        Some(num - 1)
    }
}

pub fn ask_confirmation() -> bool {
    print!("> ");
    io::stdout().flush().expect("can't flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error!");

    let trimmed = input.trim().to_lowercase();

    if trimmed == "y" || trimmed == "yes" {
        return true;
    }
    false
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
