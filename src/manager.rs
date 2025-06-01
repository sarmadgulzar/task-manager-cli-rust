use crate::storage::{BoxedStorage, StorageError};
use crate::task::{Task, TaskStatus};
use crate::cli::TaskStatusShortcut;

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

    pub fn complete_task(&mut self, id: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.status = TaskStatus::Complete;
            true
        } else {
            false
        }
    }

    pub fn update_task_status(&mut self, id_prefix: &str, status_shortcut: TaskStatusShortcut) -> Result<(), String> {
        // Find task by prefix
        let task_id = {
            let task = self.find_task_by_prefix(id_prefix)?;
            task.id.clone()
        };

        // Update the task status
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == task_id) {
            task.status = match status_shortcut {
                TaskStatusShortcut::T => TaskStatus::Todo,
                TaskStatusShortcut::P => TaskStatus::InProgress,
                TaskStatusShortcut::C => TaskStatus::Complete,
            };
            Ok(())
        } else {
            Err(format!("Task with ID '{}' not found", id_prefix))
        }
    }

    pub fn delete_task(&mut self, id_prefix: &str) -> Result<(), String> {
        // Find task by prefix
        let task_id = {
            let task = self.find_task_by_prefix(id_prefix)?;
            task.id.clone()
        };

        // Remove the task
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != task_id);
        
        if self.tasks.len() < initial_len {
            Ok(())
        } else {
            Err(format!("Task with ID '{}' not found", id_prefix))
        }
    }

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
