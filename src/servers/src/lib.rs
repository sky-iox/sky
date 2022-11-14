// This crate will support mysql, pg protocol etc.

mod error;
mod mysql;
mod server;

pub use crate::error::Error;
pub use crate::error::Result;
pub use crate::mysql::server::MysqlServer;
