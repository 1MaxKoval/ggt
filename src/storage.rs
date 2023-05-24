use serde::{Serialize, Deserialize};
use serde_json::Value;

trait Identifiable {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn get_namespace(&self) -> &str;
    fn set_namespace(&mut self, namespace: &str);
}

#[derive(Serialize, Deserialize)]
struct Slot {
    id: usize,
    namespace: &str,
    topic_ids: Vec<usize>,
    start_time: String,
    end_time: String
}

impl Identifiable for Slot {
    fn get_id(&self) -> usize { &self.id }
    fn set_id(&mut self, id: usize) { &self.id = id; }
    fn get_namespace(&self) -> &str { &self.namespace }
    fn set_namespace(&mut self, namespace: &str) { &self.namespace = namespace; }
}

#[derive(Serialize, Deserialize)]
struct Topic {
    id: usize,
    namespace: &str,
    name: String,
    desrciption: String
}

impl Identifiable for Topic {
    fn get_id(&self) -> usize { &self.id }
    fn set_id(&mut self, id: usize) { &self.id = id; }
    fn get_namespace(&self) -> &str { &self.namespace }
    fn set_namespace(&mut self, namespace: &str) { &self.namespace = namespace; }
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

    pub fn get_object<T: ?Sized + Deserialize + Identifiable>(&self, object: T) -> T 
    where
        T: ?Sized + Deserialize + Identifiable
    {
    // Deserialize on return!
    }

    pub fn delete_object(&mut self, object: T) 
    where
        T: ?Sized + Identifiable
    {
    // Throw error?
    }

}