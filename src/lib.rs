extern crate serde;
extern crate ron;

pub use error::Result;
pub use kv::KvStore;

mod error;
mod kv;
