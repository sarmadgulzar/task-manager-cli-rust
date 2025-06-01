pub mod csv_storage;
pub mod json_storage;

use csv;
use serde_json;
use std::io;

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
