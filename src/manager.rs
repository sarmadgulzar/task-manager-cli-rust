use crate::task::{Task, TaskStatus};

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let task = Task::create(description);
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, id: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.status = TaskStatus::Complete;
            true
        } else {
            false
        }
    }

    pub fn list_tasks(&self) {
        println!(
            "{:<8} {:<20} {:<12} {}",
            "ID", "Description", "Status", "Created"
        );
        println!("{}", "-".repeat(60));

        for task in self.tasks.iter() {
            let truncated_desc = if task.description.len() > 18 {
                format!("{}...", &task.description[..15])
            } else {
                task.description.clone()
            };

            println!(
                "{:<8} {:<20} {:<12} {}",
                task.id,
                truncated_desc,
                format!("{:?}", task.status),
                task.created.format("%Y-%m-%d %H:%M")
            );
        }
    }
}
