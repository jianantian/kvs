extern crate serde;
extern crate serde_json;

pub use error::Result;
pub use kv::KvStore;

mod error;
mod kv;
mod utils;
