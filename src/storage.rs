use csv;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

use crate::task::Task;

#[derive(Debug)]
pub enum StorageError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    CsvError(csv::Error),
}

impl From<io::Error> for StorageError {
    fn from(error: io::Error) -> Self {
        StorageError::IoError(error)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(error: serde_json::Error) -> Self {
        StorageError::JsonError(error)
    }
}

impl From<csv::Error> for StorageError {
    fn from(error: csv::Error) -> Self {
        StorageError::CsvError(error)
    }
}

pub trait Storage<T> {
    type Error;
    fn save(&self, data: &[T]) -> Result<(), Self::Error>;
    fn load(&self) -> Result<Vec<T>, Self::Error>;
}

pub struct JsonStorage {
    file_path: String,
}

impl JsonStorage {
    pub fn new(file_path: String) -> Self {
        JsonStorage { file_path }
    }
}

impl<T> Storage<T> for JsonStorage
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = StorageError;

    fn save(&self, data: &[T]) -> Result<(), Self::Error> {
        let json_string = serde_json::to_string_pretty(data)?;
        fs::write(&self.file_path, json_string)?;
        Ok(())
    }
    fn load(&self) -> Result<Vec<T>, Self::Error> {
        match std::fs::read_to_string(&self.file_path) {
            Ok(content) => {
                let data: Vec<T> = serde_json::from_str(&content)?;
                Ok(data)
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(err) => Err(StorageError::IoError(err)),
        }
    }
}

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
