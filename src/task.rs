use crate::random;

use chrono::{DateTime, Utc};
use random::generate_id;

#[derive(Debug)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Complete,
}

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub created: DateTime<Utc>,
}

impl Task {
    pub fn new(
        id: String,
        description: String,
        status: TaskStatus,
        created: DateTime<Utc>,
    ) -> Self {
        Task {
            id,
            description,
            status,
            created,
        }
    }

    pub fn create(description: String) -> Self {
        Task::new(generate_id(), description, TaskStatus::Todo, Utc::now())
    }
}
