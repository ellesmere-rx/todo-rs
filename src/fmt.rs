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
    show    [index]    Show all tasks or task by index
    mark    <index>    Mark task as done
    rm      <index>    Delete task by index
    save    <path>     Save tasks to file
    load    <path>     Load tasks from file
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

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}