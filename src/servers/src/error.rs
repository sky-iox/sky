use std::net::SocketAddr;

use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("IO error, source: {}", source))]
    IO { source: std::io::Error },

    #[snafu(display("Failed to bind address: {}", addr))]
    TcpBind {
        addr: SocketAddr,
        source: std::io::Error,
        backtrace: Backtrace,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
