use crate::presentation::usage_of_types;

pub struct Storage<T> {
    pub data: T,
    pub filename: &'static str
}

impl<T> Storage<T> {
    pub fn save(&self){
        println!("Saving {}", self.filename);
    }
}

pub fn storage_usage() {
    let storage = Storage {data:usage_of_types(), filename:"file.txt" };
    storage.save();
}