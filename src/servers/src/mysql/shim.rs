use std::io;

use opensrv_mysql::{AsyncMysqlShim, OkResponse, QueryResultWriter, StatementMetaWriter};
use tokio::io::AsyncWrite;

pub struct MysqlShim {}

#[async_trait::async_trait]
impl<W: AsyncWrite + Send + Unpin> AsyncMysqlShim<W> for MysqlShim {
    type Error = io::Error;

    async fn on_prepare<'a>(
        &'a mut self,
        _: &'a str,
        info: StatementMetaWriter<'a, W>,
    ) -> io::Result<()> {
        info.reply(42, &[], &[]).await
    }

    async fn on_execute<'a>(
        &'a mut self,
        _: u32,
        _: opensrv_mysql::ParamParser<'a>,
        results: QueryResultWriter<'a, W>,
    ) -> io::Result<()> {
        results.completed(OkResponse::default()).await
    }

    async fn on_close(&mut self, _: u32) {
        println!("on close!");
    }

    async fn on_query<'a>(
        &'a mut self,
        _sql: &'a str,
        results: QueryResultWriter<'a, W>,
    ) -> io::Result<()> {
        results.start(&[]).await?.finish().await
    }
}
