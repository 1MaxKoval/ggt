use std::fmt;

pub enum ErrorType {
    File(&str), // file path
    Json, 
}

#[derive(Debug, Clone)]
pub struct StorageError {
    type: ErrorType,
    message: &str
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, &self.message) // Display different message based on its type?
    }
}

