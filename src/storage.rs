use serde::{Serialize, Deserialize};
use serde_json::Value;

trait Identifiable {
    fn get_id(&self) -> usize;
    fn set_id(&mut self);
}

#[derive(Serialize, Deserialize)]
struct Slot {
    id: usize,
    topic_ids: Vec<usize>,
    start_time: String,
    end_time: String
}

impl Identifiable for Slot {
    fn get_id(&self) -> usize { &self.id }
    fn set_id(&mut self, id: usize) -> usize { &self.id = id; }
}

#[derive(Serialize, Deserialize)]
struct Topic {
    id: usize,
    name: String,
    desrciption: String
}

impl Identifiable for Topic {
    fn get_id(&self) -> usize { &self.id }
    fn set_id(&mut self, id: usize) -> usize { &self.id = id; }
}

struct Storage {
    parsed_data: Value
}


impl Storage {

    pub fn new(location: &str) -> Storage {
    }

    pub fn write_object<T>(&mut self, object: T) -> T 
    where
        T: ?Sized + Serialize + Identifiable,
    {
    // Assign id + spit out new version of the object
    }

    pub fn get_object<T: ?Sized + Deserialize + Identifiable>(&self, namespace: &str, object_id: usize) -> T 
    where
        T: Sized + Deserialize + Identifiable
    {
    // Deserialize on return!
    }

    pub fn delete_object(&mut self, namespace: &str, object_id: usize) -> bool {
    }

}