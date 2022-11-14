use std::net::SocketAddr;

use crate::error::Result;

#[async_trait::async_trait]
pub trait Server {
    async fn start(&mut self, addr: SocketAddr) -> Result<()>;

    async fn shutdown(&mut self) -> Result<()>;
}
