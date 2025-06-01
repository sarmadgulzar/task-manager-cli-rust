use super::{Storage, StorageError};
use crate::task::Task;
use csv;
use std::fs;
use std::io;

pub struct CsvStorage {
    file_path: String,
}

impl CsvStorage {
    pub fn new(file_path: String) -> Self {
        CsvStorage { file_path }
    }
}

impl Storage<Task> for CsvStorage {
    type Error = StorageError;

    fn save(&self, data: &[Task]) -> Result<(), Self::Error> {
        let mut writer = csv::Writer::from_path(&self.file_path)?;

        for task in data {
            writer.serialize(task)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn load(&self) -> Result<Vec<Task>, Self::Error> {
        let file = match fs::File::open(&self.file_path) {
            Ok(file) => file,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Ok(Vec::new());
            }
            Err(err) => return Err(StorageError::IoError(err)),
        };

        let mut reader = csv::Reader::from_reader(file);
        let mut tasks = Vec::new();

        for result in reader.deserialize() {
            let task: Task = result?;
            tasks.push(task);
        }

        Ok(tasks)
    }
}
