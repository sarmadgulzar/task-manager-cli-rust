mod manager;
mod random;
mod storage;
mod task;

use manager::TaskManager;
use storage::csv_storage::CsvStorage;

fn main() {
    let storage = CsvStorage::new("tasks.csv".to_string());
    let mut task_manager = TaskManager::new(storage);

    if let Err(e) = task_manager.load() {
        println!("Could not load tasks: {:?}", e);
    }
    task_manager.show_tasks();
}
