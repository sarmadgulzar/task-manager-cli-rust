mod cli;
mod manager;
mod random;
mod storage;
mod task;

use cli::Command;
use manager::TaskManager;
use storage::create_storage;

fn main() {
    let storage = create_storage();
    let mut task_manager = TaskManager::new(storage);
    let _ = task_manager.load();

    match cli::parse_args() {
        Ok(Command::List) => task_manager.show_tasks(),
        Ok(Command::Add(desc)) => println!("add {}", desc),
        Ok(Command::WithId { id, action }) => println!("id action {} {:?}", id, action),
        Err(e) => println!("Error: {}", e),
    }
}
