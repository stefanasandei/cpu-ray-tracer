use std::collections::HashMap;
use std::time::{Instant};

pub struct Profiler {
    data: HashMap<String, Instant>
}

pub fn create() -> Profiler {
    return Profiler {
        data: HashMap::new()
    };
}

impl Profiler {    
    pub fn start(&mut self, name: &str) {
        self.data.insert(name.to_string(), Instant::now());
    }

    pub fn end(&mut self, name: &str) {
        if self.data.contains_key(name) {
            println!("{}: {:?}", name, self.data[name].elapsed());
            self.data.remove(name);
        }
    }
}