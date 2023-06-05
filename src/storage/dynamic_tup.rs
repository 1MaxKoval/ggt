use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DynamicTup {
    bound: Vec<usize>
}

impl DynamicTup {

    pub fn new(a: Vec<usize>) -> DynamicTup {
        match a.len() {
            1 | 2 => return DynamicTup { bound: a },
            _ => panic!("Ranged tuple can only be initialised to a vec of size 1 or 2!")
        }
    }

    pub fn len(&self) -> usize { self.bound.len() }

    pub fn upper(&self) -> usize {
        match self.bound.len() {
            2 => return self.bound[1],
            _ => return self.bound[0]
        }
    }

    pub fn rm_upper(&mut self)  {
        if self.bound.len() == 2 {
            self.bound.pop();
        } else { panic!("Unable to remove an upper bound while being a unit tuple"); }
    }

    pub fn set_upper(&mut self, i: usize) {
        if self.bound.len() == 1 {
            self.bound.push(i);
        } else { self.bound[1] = i; }
    }

    pub fn lower(&self) -> usize { return self.bound[0]; }

    pub fn rm_lower(&mut self) {
        if self.bound.len() == 2 { 
            self.bound.remove(0);
        } else { panic!("Unable to remove a lower bound while being a unit tuple"); }
    }

    pub fn set_lower(&mut self, i: usize) {
        if self.bound.len() == 1 {
            self.bound.insert(0, i);
        }
        else { self.bound[0] = i; }
    }

}