use std::net::SocketAddr;

use opensrv_mysql::AsyncMysqlIntermediary;
use snafu::ResultExt;
use stream_cancel::{Trigger, Valve};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_stream::StreamExt;
use tokio_util::io::{ReaderStream, StreamReader};

use crate::error;
use crate::error::Result;
use crate::server::Server;

use super::shim::MysqlShim;

pub struct MysqlServer {
    valve: (Option<Trigger>, Valve),
}

impl MysqlServer {
    pub fn new() -> MysqlServer {
        let (trigger, value) = Valve::new();

        MysqlServer {
            valve: (Some(trigger), value),
        }
    }
}

impl Default for MysqlServer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Server for MysqlServer {
    async fn start(&mut self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(addr)
            .await
            .context(error::TcpBindSnafu { addr })?;

        let (_, valve) = &self.valve;
        let mut incoming = valve.wrap(TcpListenerStream::new(listener));

        let value_clone = valve.clone();
        while let Some(stream) = incoming.next().await.transpose().context(error::IOSnafu)? {
            let (read, sink) = stream.into_split();
            let read = value_clone.wrap(ReaderStream::new(read));
            tokio::spawn(async move {
                AsyncMysqlIntermediary::run_on(MysqlShim {}, StreamReader::new(read), sink).await
            });
        }
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        if self.valve.0.take().is_none() {
            // TODO: remove println
            println!("server is shutdown already!");
        }
        Ok(())
    }
}
