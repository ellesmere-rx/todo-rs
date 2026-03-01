mod fmt;

fn main() {
    fmt::print_menu();
    let (command, args) = fmt::read_prompt();
    println!("[{}]\n[{}]", command, args);
    match command.as_str() {
            "add" => {
                if !args.is_empty() {
                    println!("You called: {} with args: {}!", command, args);
                } else {
                    println!("You called: {} with no args!", command);
                }
                
            }
            _ => {
                println!("Unknown")
            }
        }
    }
