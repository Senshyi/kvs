use failure::Fail;

/// Describes all errors that can happen in Kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    /// Io error
    #[fail(display = "Failed to IO")]
    Io(),
}
