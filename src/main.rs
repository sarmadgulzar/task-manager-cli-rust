mod cli;
mod manager;
mod random;
mod storage;
mod task;

use cli::{Command, IdAction};
use manager::TaskManager;
use storage::create_storage;

fn main() {
    let storage = create_storage();
    let mut task_manager = TaskManager::new(storage);
    let _ = task_manager.load();

    match cli::parse_args() {
        Ok(Command::List) => task_manager.show_tasks(),
        Ok(Command::Add(description)) => {
            task_manager.add_task(description);
            let _ = task_manager.save();
        }
        Ok(Command::WithId { id, action }) => {
            match action {
                IdAction::Delete => {
                    match task_manager.delete_task(&id) {
                        Ok(()) => {
                            println!("Task deleted successfully");
                            let _ = task_manager.save();
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                IdAction::Status(status_shortcut) => {
                    match task_manager.update_task_status(&id, status_shortcut) {
                        Ok(()) => {
                            let status_name = match status_shortcut {
                                cli::TaskStatusShortcut::T => "Todo",
                                cli::TaskStatusShortcut::P => "In Progress",
                                cli::TaskStatusShortcut::C => "Complete",
                            };
                            println!("Task status updated to: {}", status_name);
                            let _ = task_manager.save();
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
