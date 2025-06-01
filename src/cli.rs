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

#[derive(Debug, Clone, Copy)]
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
        3 => {
            // "./tm <command> <arg>"
            match args[1].as_str() {
                "add" => Ok(Command::Add(args[2].clone())),
                _ => match args[2].as_str() {
                    "delete" => Ok(Command::WithId {
                        id: args[1].clone(),
                        action: IdAction::Delete,
                    }),
                    "t" => Ok(Command::WithId {
                        id: args[1].clone(),
                        action: IdAction::Status(TaskStatusShortcut::T),
                    }),
                    "p" => Ok(Command::WithId {
                        id: args[1].clone(),
                        action: IdAction::Status(TaskStatusShortcut::P),
                    }),
                    "c" => Ok(Command::WithId {
                        id: args[1].clone(),
                        action: IdAction::Status(TaskStatusShortcut::C),
                    }),
                    _ => Err(format!("Unknown command: {}", args[1])),
                },
            }
        }
        _ => {
            // More than 3 arguments - handle add command with multiple words
            match args[1].as_str() {
                "add" => {
                    // Join all arguments after "add" into a single task description
                    let description = args[2..].join(" ");
                    Ok(Command::Add(description))
                }
                _ => Err(format!("Unknown command: {}", args[1])),
            }
        }
    }
}
