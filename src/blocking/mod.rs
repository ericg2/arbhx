use std::io;
use std::io::{Read, Seek, Write};

mod operator;
mod reader;
mod writer;
mod full;
mod query;
mod file;

pub use {
    operator::Operator,
    query::DataQuery,
    file::DataFile
};

pub trait CompatRead: Read + Seek + Send + Sync + 'static {}

pub trait CompatAppend: Write + Send + Sync + 'static {
    fn close(&mut self) -> io::Result<()>;
}

pub trait CompatFull: CompatRead + CompatAppend {}

pub type FileIterator = dyn Iterator<Item = io::Result<DataFile>> + Send;
