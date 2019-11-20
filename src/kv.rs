use ron;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            data: HashMap::new(),
        }
    }

    pub fn open(tmpdir: &Path) -> Result<Self, String> {
        let mut file = match File::open(tmpdir) {
            Ok(file) => file,
            Err(why) => return Result::Err(why.to_string()),
        };

        let mut b: Vec<u8> = vec![];
        match file.read(&mut b) {
            Ok(_) => (),
            Err(why) => return Result::Err(why.to_string()),
        };

        let store = match ron::de::from_bytes(&b) {
            Ok(res) => Result::Ok(res),
            Err(why) => Result::Err(why.to_string()),
        };
        store
    }

    pub fn get(&self, key: String) -> Result<Option<String>, String> {
        Result::Ok(self.data.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        match self.data.insert(key, value) {
            Some(_) => Result::Ok(()),
            _ => Result::Err("Error".to_owned()),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<(), String> {
        match self.data.remove(&key) {
            Some(_) => Result::Ok(()),
            _ => Result::Err("Error".to_owned()),
        }
    }
}
