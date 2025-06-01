use crate::storage::{BoxedStorage, StorageError};
use crate::task::{Task, TaskStatus};

pub struct TaskManager<S> {
    tasks: Vec<Task>,
    storage: S,
}

impl TaskManager<BoxedStorage> {
    pub fn new(storage: BoxedStorage) -> Self {
        TaskManager {
            tasks: Vec::new(),
            storage,
        }
    }

    pub fn save(&self) -> Result<(), StorageError> {
        self.storage.save(&self.tasks)
    }

    pub fn load(&mut self) -> Result<(), StorageError> {
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

    pub fn find_task_by_prefix(&self, prefix: &str) -> Result<&Task, String> {
        let matches: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.id.starts_with(prefix))
            .collect();

        match matches.len() {
            0 => Err(format!("No task found with ID starting with '{}'", prefix)),
            1 => Ok(matches[0]),
            _ => {
                let suggestions: Vec<String> = matches
                    .iter()
                    .map(|task| format!("  {} - {}", &task.id[..4], task.description))
                    .collect();

                Err(format!(
                    "Ambiguous ID '{}'. Multiple tasks match:\n{}\nPlease be more specific.",
                    prefix,
                    suggestions.join("\n")
                ))
            }
        }
    }
}
