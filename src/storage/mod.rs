mod exceptions;

use serde::{Serialize, Deserialize};
use serde_json::{ Value };
use exceptions::{StorageError, ErrorType};
use std::fs::File;
use std::io;

trait Identifiable {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn get_namespace(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
struct Slot {
    id: usize,
    topic_ids: Vec<usize>,
    start_time: String,
    end_time: String
}

impl Identifiable for Slot {
    fn get_id(&self) -> usize { self.id }
    fn set_id(&mut self, id: usize) { self.id = id; }
    fn get_namespace(&self) -> &str { "Slot" }
}

#[derive(Serialize, Deserialize)]
struct Topic {
    id: usize,
    name: String,
    desrciption: String
}

impl Identifiable for Topic {
    fn get_id(&self) -> usize { self.id }
    fn set_id(&mut self, id: usize) { self.id = id; }
    fn get_namespace(&self) -> &str { "Topic" }
}

struct Storage {
    parsed_data: Value
}

struct Query {
    id: usize,
    namespace: String
}

impl Storage {

    pub fn new(path: String) -> Result<Storage, StorageError>  {
        let reader_result = File::open(&path);
        let reader = match reader_result {
            Ok(a) => a,
            Err(err) => {
                return Err(StorageError {
                    kind: ErrorType::File(path.clone()),
                    message: format!("Unable to read file with path {path}!")
                });
            }
        };
        let json_result = serde_json::from_reader(reader);
        let json_value = match json_result {
            Ok(v) => v,
            Err(e) => {
                return Err(StorageError {
                    kind: ErrorType::Json,
                    message: format!("Unable to deserialize JSON with file path {path}!")
                });
            }
        };
        Ok(Storage {
            parsed_data: json_value
        })
    }

    pub fn add_object<T>(&mut self, object: T)
    where
        T: Serialize + Identifiable,
    {
    // Assign id + spit out new version of the object
    }

    pub fn get_object<G>(&self, query: &Query) -> Value  {
    // Deserialize on return or throw!
    // TODO: Update error handling and add proper deserialization once learnt!
    // Dont know how to implement a generic method for deserialization!
    }

    pub fn delete_object<T>(&mut self, query: &Query) {
    // Throw error?
    // What if a topic gets deleted?
    }

}