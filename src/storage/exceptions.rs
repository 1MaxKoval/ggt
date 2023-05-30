use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorType {
    File(String), // file path
    Json, 
}

#[derive(Debug, Clone)]
pub struct StorageError {
    kind: ErrorType,
    message: String
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message) // Display different message based on its type?
    }
}

