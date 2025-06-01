use crate::storage::Storage;
use crate::task::{Task, TaskStatus};

pub struct TaskManager<S> {
    tasks: Vec<Task>,
    storage: S,
}

impl<S> TaskManager<S>
where
    S: Storage<Task>,
{
    pub fn new(storage: S) -> Self {
        TaskManager {
            tasks: Vec::new(),
            storage,
        }
    }

    pub fn save(&self) -> Result<(), S::Error> {
        self.storage.save(&self.tasks)
    }

    pub fn load(&mut self) -> Result<(), S::Error> {
        self.tasks = self.storage.load()?;
        Ok(())
    }

    pub fn add_task(&mut self, description: String) {
        let task = Task::create(description);
        self.tasks.push(task);
    }

    #[allow(dead_code)]
    pub fn complete_task(&mut self, id: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.status = TaskStatus::Complete;
            true
        } else {
            false
        }
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    #[allow(dead_code)]
    pub fn show_tasks(&self) {
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
