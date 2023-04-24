
use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError};

pub async fn connection()  -> Result<(Conn, Pool), mysql_async::Error> {
let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock")?;
let pool = Pool::new(db_url);
let mut conn = pool.get_conn().await?;

// pass the connection and the pool to the caller
Ok((conn, pool))
} 