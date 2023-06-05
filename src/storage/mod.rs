mod exceptions;
mod dynamic_tup;

use serde::{Serialize, Deserialize};
use serde_json::{Value, to_value};
use exceptions::{StorageError, ErrorType};
use dynamic_tup::DynamicTup;
use std::fs::File;
use std::collections::HashMap;

trait Identifiable {
    fn get_id(&self) -> Option<usize>;
    fn set_id(&mut self, id: Option<usize>);
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
    id_pools: HashMap<String, Vec<DynamicTup>>,
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

    fn save(&self) {}

    fn deallocate_id() {}

    fn allocate_id(&mut self, namespace: &str) -> usize {
        // #TODO: This will break if vec.len() > usize
        let pools = self.config.id_pools[namespace];
        let l = pools.len();
        if l == 0 {
            pools.push(DynamicTup::new(vec![0]));
            return 0;
        }
        // Ensure that c is the first lower bound
        let mut to_remove: Vec<usize> = vec![];
        for (i, x) in pools.iter().enumerate() {
            match i {
                0 => {
                    if x.lower() != 0 {
                        x.set_lower(x.lower() - 1);
                        return x.lower();
                    }
                },
                (l - 1) => {
                    if x.upper() != usize::MAX {
                        x.set_upper(x.upper() + 1); 
                        return x.upper();
                    }
                },
                _ => {
                    let n = pools[i + 1];
                    let d = n.lower() - x.upper();
                    if d > 1 {
                        x.set_upper(x.upper() + 1);
                        return x.upper();
                    } 
                    x.set_upper(n.upper());
                    // TODO: Remove the array that follows! Deleting from an array while iterating through it hmmmm
                }
            }
        }
        panic!("Something went wrong!");
    } 
    
    pub fn add_object<T>(&mut self, object: T) -> Result<Value, StorageError> //TODO: Change the return to type to generic for better abstraction!
    where
        T: Serialize + Identifiable, // + Deserialize later
    {
        let namespace = object.get_namespace();
        match object.get_id() {
            Some(i) => {
               // update object
                let Ok(v) = to_value(object) else {
                    return Err(StorageError { 
                        kind: ErrorType::Json, 
                        message: format!("Unable to serialise JSON object being added \"{namespace}\" namespace").to_string()
                    })
                };
                self.data[namespace][i] = v;
                Ok(self.data[namespace][i])
             },
            None => {

            }
        }
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



