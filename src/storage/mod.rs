mod exceptions;

use serde::{Serialize, Deserialize};
use serde_json::{ Value };
use exceptions::{StorageError, ErrorType};
use std::fs::File;
use std::io;
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize)]
struct Config {
    id_pools: HashMap<String, Vec<Vec<usize>>>
}

#[derive(Serialize, Deserialize)]
struct Storage {
    config: Config,
    data: HashMap<String, Value>
}

struct Query {
    id: usize,
    namespace: String
}

impl Storage {

    pub fn from(path: &str) -> Result<Storage, StorageError>  {
        let reader_result = File::open(&path);
        let reader = match reader_result {
            Ok(a) => a,
            Err(err) => {
                return Err(StorageError {
                    kind: ErrorType::File(String::from(path)),
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
        let storage: Storage = json_value;
        Ok(storage)
    }

    fn get_unused_id(namespace: &str) -> usize {
        3
    } 
    
    pub fn add_object<T>(&mut self, object: T) -> Value //TODO: Change the return to type to generic for better abstraction!
    where
        T: Serialize + Identifiable,
    {
        
    }

    pub fn get_object<G>(&self, query: &Query) -> Value { //TODO: Change the return to type to generic for better abstraction!

    }

    pub fn delete_object<T>(&mut self, query: &Query) {
    // Throw error?
    // What if a topic gets deleted?
    }

}

fn indx<'a, 'b>(a: &'a Value, s: &'b str) -> Result<Value, StorageError> {
    match a[s] {
        Value::Null => {
            return Err(StorageError {
                kind: ErrorType::Json,
                message: format!("Could not index into JSON object with key {s}!")
            });
        },
        _ => {
            return Ok(a[s]);
        }
    };
}



