
use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError, Row};
use crate::models::user::User;
pub mod queries;

pub async fn connection() -> Result<Pool, mysql_async::Error> {
let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock")?;
let pool = Pool::new(db_url);

// pass the connection and the pool to the caller
Ok(pool)
} 