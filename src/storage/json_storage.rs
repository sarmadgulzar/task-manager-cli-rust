use super::{Storage, StorageError};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

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
