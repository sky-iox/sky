use std::io;

use opensrv_mysql::{AsyncMysqlShim, OkResponse, QueryResultWriter, StatementMetaWriter};
use tokio::io::AsyncWrite;

use sha1::Digest;
use sha1::Sha1;

pub struct MysqlShim {}

#[async_trait::async_trait]
impl<W: AsyncWrite + Send + Unpin> AsyncMysqlShim<W> for MysqlShim {
    type Error = io::Error;

    // https://dev.mysql.com/doc/dev/mysql-server/latest/page_protocol_connection_phase_authentication_methods_native_password_authentication.html
    async fn authenticate(
        &self,
        _auth_plugin: &str,
        _username: &[u8],
        _salt: &[u8],
        auth_data: &[u8],
    ) -> bool {
        // hashed_pwd will store in the sys.user table
        // hashed_pwd = sha1(sha1(pwd))
        let hashed_pwd = calc_sha1(&calc_sha1(b"123456"));

        // auth
        let mut hasher = Sha1::new();
        hasher.update(";X,po_k}>o6^Wz!/kM}N".as_bytes());
        hasher.update(hashed_pwd);
        let result = hasher.finalize();

        let one_hashed = calc_sha1(b"123456");
        let mut vec = Vec::new();
        for i in 0..result.len() {
            vec.push(result[i] ^ one_hashed[i]);
        }

        println!("{:?}", auth_data);
        println!("{:?}", &vec[0..20]);
        auth_data == &vec[0..20]
    }

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

fn calc_sha1(v: &[u8]) -> [u8; 20] {
    let mut m = sha1::Sha1::new();

    m.update(v);
    m.finalize().into()
}
