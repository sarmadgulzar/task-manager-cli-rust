mod manager;
mod random;
mod task;

use chrono::Utc;
use manager::TaskManager;
use task::{Task, TaskStatus};

fn main() {
    let mut task_manager = TaskManager::new();
    task_manager.add_task("Buy groceries".to_string());
    task_manager.add_task("Learn Rust traits".to_string());
    task_manager.add_task("Go for a run".to_string());

    task_manager.list_tasks();
}
