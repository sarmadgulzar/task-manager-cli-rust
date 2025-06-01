pub enum Command {
    List,
    Add(String),
    WithId { id: String, action: IdAction },
}

#[derive(Debug)]
pub enum IdAction {
    Delete,
    Status(TaskStatusShortcut),
}

#[derive(Debug)]
pub enum TaskStatusShortcut {
    T,
    P,
    C,
}

pub fn parse_args() -> Result<Command, String> {
    let args: Vec<String> = std::env::args().collect();

    // Skip program name (first arg is always the executable path)
    match args.len() {
        1 => {
            // Just "./tm" with no arguments
            Ok(Command::List)
        }
        2 => {
            // "./tm <something>"
            match args[1].as_str() {
                "list" => Ok(Command::List),
                _ => Err(format!("Unknown command: {}", args[1])),
            }
        }
        _ => {
            // More than 2 arguments - handle later
            Err("Too many arguments".to_string())
        }
    }
}
