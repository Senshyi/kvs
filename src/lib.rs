#![deny(missing_docs)]
//! A key value store that enables persistant storage

pub use errors::KvsError;
pub use kvs::{KvStore, Result};

mod errors;
mod kvs;
