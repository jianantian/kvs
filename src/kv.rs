use crate::utils::{BufReaderWithPos, BufWriterWithPos};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};
use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::OsStr;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

// value type (filename, file offset, value size)
#[derive(Debug)]
struct Value {
    gen: u64,
    pos: u64,
    size: u64,
}

// Store key value relation in memory
// value is of type Value
pub struct KvStore {
    path: String,
    index: HashMap<String, Value>,
    readers: HashMap<u64, BufReaderWithPos<File>>,
    writer: BufWriterWithPos<fs::File>,
    current_gen: u64,
}

// command record to write in log file
#[derive(Serialize, Deserialize)]
enum Record {
    SetRecord { key: String, value: String },
    RemoveRecord { key: String },
}

impl KvStore {
    pub fn open(tmpdir: &Path) -> Result<KvStore, String> {
        match fs::create_dir_all(&tmpdir) {
            io::Result::Ok(_) => (),
            io::Result::Err(why) => return Result::Err(why.to_string()),
        };
        // println!("open diretory: {:?}", tmpdir);
        let mut index = HashMap::new();
        let mut readers = HashMap::new();

        let gen_list = get_gen_list(tmpdir)?;
        // println!("gen list: {:?}", &gen_list);
        for gen in &gen_list {
            let fname = gen_fname(tmpdir, gen.clone());
            let mut reader = match new_reader(&fname) {
                Ok(_x) => _x,
                Err(why) => return Result::Err(why.to_string()),
            };
            match load_file(gen.clone(), &mut index, &mut reader) {
                Ok(_) => (),
                Err(why) => return Result::Err(why.to_string()),
            };
            readers.insert(gen.clone(), reader);
        }

        let current_gen = gen_list.last().unwrap_or(&0) + 1;
        // println!("next gen: {}", current_gen);
        let writer = new_log_file(tmpdir, current_gen, &mut readers)?;

        Ok(KvStore {
            path: tmpdir.display().to_string(),
            index: index,
            readers: readers,
            writer: writer,
            current_gen: current_gen,
        })
    }

    fn write_record(&mut self, record: Record) -> Result<Option<Value>, String> {
        let pos = self.writer.pos;
        match serde_json::to_writer(&mut self.writer, &record) {
            std::result::Result::Ok(_) => (),
            std::result::Result::Err(why) => return Result::Err(why.to_string()),
        };
        match self.writer.flush() {
            io::Result::Err(why) => return Result::Err(why.to_string()),
            _ => (),
        };
        Ok(match record {
            Record::SetRecord { key: _, value: _ } => {
                // println!(
                //     "write record: gen: {}, pos: {}, size: {}",
                //     self.current_gen,
                //     pos,
                //     self.writer.pos - pos
                // );
                Some(Value {
                    gen: self.current_gen,
                    pos: pos,
                    size: self.writer.pos - pos,
                })
            }
            Record::RemoveRecord { key: _ } => None,
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>, String> {
        // println!("prepare to get key: {}", &key);
        match self.index.get(&key) {
            Some(value) => {
                // println!("get key: {}, value: {:?}", &key, &value);
                let gen = value.gen;
                let v_pos = value.pos;
                let v_size = value.size;
                let reader = self.readers.get_mut(&gen).unwrap();
                match reader.seek(SeekFrom::Start(v_pos)) {
                    std::result::Result::Ok(_) => (),
                    std::result::Result::Err(why) => return Result::Err(why.to_string()),
                }
                let mut record_reader = reader.take(v_size);
                match serde_json::from_reader(&mut record_reader) {
                    Ok(record) => match record {
                        Record::SetRecord { key: _, value } => {
                            // println!("read sucess, value is : {}", &value);
                            Ok(Some(value))
                        }
                        _ => Result::Err("Error Reocrd type!".to_owned()),
                    },
                    Err(why) => return Result::Err(why.to_string()),
                }
            }
            None => Ok(None),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        let v = match self.write_record(Record::SetRecord {
            key: key.clone(),
            value: value,
        })? {
            Some(x) => x,
            None => return Result::Err("Error Command!".to_owned()),
        };
        // println!("prepare to insert key: {}, value: {:?}", &key, &v);
        self.index.insert(key.clone(), v);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<(), String> {
        match self.write_record(Record::RemoveRecord { key: key.clone() })? {
            Some(_) => return Result::Err("Error Command!".to_owned()),
            None => (),
        };

        match self.index.remove(&key) {
            Some(_) => Result::Ok(()),
            _ => Result::Err(format!("Remove key: {} Error", &key).to_owned()),
        }
    }

    pub fn new_log_file(&mut self, gen: u64) -> Result<BufWriterWithPos<File>, String> {
        new_log_file(
            &gen_fname(Path::new(&self.path), gen),
            gen,
            &mut self.readers,
        )
    }
}

fn new_log_file(
    path: &Path,
    gen: u64,
    readers: &mut HashMap<u64, BufReaderWithPos<File>>,
) -> Result<BufWriterWithPos<File>, String> {
    fn _new_log_file(
        path: &Path,
        gen: u64,
        readers: &mut HashMap<u64, BufReaderWithPos<File>>,
    ) -> io::Result<BufWriterWithPos<File>> {
        let fname = gen_fname(path, gen);
        // println!("path: {:?}", &fname);
        let writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&fname)?,
        )?;
        readers.insert(gen, BufReaderWithPos::new(File::open(&fname)?)?);
        io::Result::Ok(writer)
    }
    match _new_log_file(path, gen, readers) {
        io::Result::Ok(writer) => Result::Ok(writer),
        io::Result::Err(why) => Result::Err(why.to_string()),
    }
}

fn load_file(
    gen: u64,
    index: &mut HashMap<String, Value>,
    reader: &mut BufReaderWithPos<File>,
) -> Result<u64, String> {
    let mut current_pos: u64 = 0;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Record>();
    while let Some(record) = stream.next() {
        let next_pos: u64 = stream.byte_offset().try_into().unwrap();
        match record {
            Ok(record_t) => match record_t {
                Record::SetRecord { key: k, value: _ } => {
                    let v_pos: u64 = current_pos;
                    let v_size: u64 = next_pos - current_pos;
                    index.insert(
                        k,
                        Value {
                            gen,
                            pos: v_pos,
                            size: v_size,
                        },
                    );
                    current_pos = next_pos;
                }
                Record::RemoveRecord { key } => {
                    index.remove(&key);
                }
            },
            Err(why) => return Result::Err(why.to_string()),
        }
    }

    Ok(current_pos)
}

fn gen_fname(dirname: &Path, gen: u64) -> PathBuf {
    dirname.join(format!("{}.log", gen))
}

fn new_reader(fname: &Path) -> Result<BufReaderWithPos<File>, String> {
    let file = match OpenOptions::new().read(true).open(fname) {
        Err(why) => return Result::Err(why.to_string()),
        Ok(file) => file,
    };
    match BufReaderWithPos::new(file) {
        io::Result::Ok(x) => Result::Ok(x),
        io::Result::Err(why) => Result::Err(why.to_string()),
    }
}

fn new_writer(fname: &Path, pos: u64) -> Result<BufWriterWithPos<File>, String> {
    let file = match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(fname)
    {
        Err(why) => return Result::Err(why.to_string()),
        Ok(file) => file,
    };
    match BufWriterWithPos::new(file) {
        io::Result::Ok(mut writer) => match &writer.seek(SeekFrom::Start(pos)) {
            Ok(_) => Result::Ok(writer),
            Err(why) => Result::Err(why.to_string()),
        },
        io::Result::Err(why) => Result::Err(why.to_string()),
    }
}

/// Returns sorted generation numbers in the given directory
fn get_gen_list(path: &Path) -> Result<Vec<u64>, String> {
    let mut gen_list: Vec<u64> = fs::read_dir(&path)
        .unwrap()
        .flat_map(|res| -> io::Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    gen_list.sort_unstable();
    Ok(gen_list)
}
