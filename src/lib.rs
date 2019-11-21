extern crate serde;
extern crate ron;
extern crate uuid;

pub use error::Result;
pub use kv::KvStore;

mod error;
mod kv;
mod utils;
