mod exceptions;

use serde::{Serialize, Deserialize};
use serde_json::{Value, to_value};
use exceptions::{StorageError, ErrorType};
use std::fs::File;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

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

struct RangedTuple {
    bound: Vec<usize>
}

impl RangedTuple {

    fn new(a: Vec<usize>) -> RangedTuple {
        match a.len() {
            1 | 2 => return RangedTuple { bound: a },
            _ => panic!("Ranged tuple can only be initialised to a vec of size 1 or 2!")
        }
    }

    fn upper(&self) -> usize {
        match self.bound.len() {
            1 => return self.bound[0],
            2 => return self.bound[1],
            _ => panic!("Uknown state for RangedTuple, exiting..."),
        }
    }

    fn rm_upper(&mut self)  {
        if self.bound.len() == 2 {
            self.bound.pop();
        }
        panic!("Unable to remove an upper bound while being a unit tuple");
    }

    fn set_upper(&mut self, i: usize) {
        if self.bound.len() == 1 {
            self.bound.push(i);
        }
    }

    fn lower(&self) -> usize {
        match self.bound.len() {
            1 | 2 => return self.bound[0],
            _ => panic!("Uknown state for RangedTuple, exiting..."),
        }
    }

    fn rm_lower(&mut self) {
        if self.bound.len() == 2 {
            self.bound.pop();
            return;
        }
        panic!("Unable to remove a lower bound while being a unit tuple");
    }

    fn set_lower(&mut self, i: usize) {
        if self.bound.len() == 1 {
            self.bound.push(i);
        }
    }


}

#[derive(Serialize, Deserialize)]
struct Config {
    id_pools: HashMap<String, Vec<Vec<usize>>>,
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

    fn save(&self) {
    }

    fn deallocate_id()

    fn allocate_id(&mut self, namespace: &str) -> usize {
        // #TODO: This will break if vec.len() > usize
        let pools = self.config.id_pools[namespace];
        if pools.len() == 0 {
            pools.push(vec![0]);
            return 0;
        }
        // Ensure that c is the first lower bound
        let c: Vec<usize>;
        for elem in pools {
            
        }
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



