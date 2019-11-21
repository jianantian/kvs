use ron;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use uuid;

use crate::utils::{BufReaderWithPos, BufWriterWithPos};

// value type (filename, file offset, value size)
type Value = (String, u64, u64);

// Store key value relation in memory
// value is of type Value
pub struct KvStore {
    index: HashMap<String, Value>,
    reader: BufReaderWithPos<fs::File>,
    writer: BufWriterWithPos<fs::File>,
    current_pos: u64,
}

// command record to write in log file
#[derive(Serialize, Deserialize)]
enum Record {
    SetRecord { key: String, value: String },
    RemoveRecord { key: String },
}

impl Record {
    fn from_file(fname: &Path) -> Result<Self, String> {
        let mut b: Vec<u8> = vec![];
        let mut file = match fs::File::open(fname) {
            Ok(file) => file,
            Err(why) => return Result::Err(why.to_string()),
        };

        match file.read(&mut b) {
            Ok(_) => (),
            Err(why) => return Result::Err(why.to_string()),
        };

        let record = match ron::de::from_bytes(&b) {
            Ok(res) => Result::Ok(res),
            Err(why) => Result::Err(why.to_string()),
        };
        record
    }
}

impl KvStore {
    pub fn open(tmpdir: &Path) -> Result<Self, String> {
        

        let dir_iter = match fs::read_dir(tmpdir) {
            Ok(_x) => _x,
            Err(why) => return Result::Err(why.to_string()),
        };
        for entry in dir_iter {
            let entry = match entry {
                Ok(_x) => _x,
                Err(why) => return Result::Err(why.to_string()),
            };
            let fname = entry.path();
            match Record::from_file(&fname) {
                Result::Ok(record) => {
                    let key = record.key;
                    // let value: Value = (&fname.display(), record.value_size, record.value)
                }
                Result::Err(why) => println!("Err reading {}: {}", &fname.display(), why),
            }
        }
        unimplemented!()
    }

    // build memory key-value storage from file
    fn build_index(&self, fnmae: &Path) {
        unimplemented!()
    }

    pub fn get(&self, key: String) -> Result<Option<Value>, String> {
        Result::Ok(self.data.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: Value) -> Result<(), String> {
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
