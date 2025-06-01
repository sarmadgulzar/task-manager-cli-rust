pub mod csv_storage;
pub mod json_storage;

use csv;
use csv_storage::CsvStorage;
use json_storage::JsonStorage;
use serde_json;
use std::io;

use crate::task::Task;

pub type BoxedStorage = Box<dyn Storage<Task, Error = StorageError>>;

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

pub fn create_storage() -> BoxedStorage {
    let storage_type = std::env::var("TM_STORAGE")
        .unwrap_or_else(|_| "json".to_string())
        .to_lowercase();

    match storage_type.as_str() {
        "csv" => Box::new(CsvStorage::new("tasks.csv".to_string())),
        "json" | _ => Box::new(JsonStorage::new("tasks.json".to_string())),
    }
}
