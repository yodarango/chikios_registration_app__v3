
use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError};

pub async fn connection()  -> Result<(Conn, Pool), mysql_async::Error> {
let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock")?;
let pool = Pool::new(db_url);
let mut conn = pool.get_conn().await?;

// let users = "SELECT ID, signature FROM users"
// .with(())
// .map(&mut conn, |(ID, signature)| User { ID, signature }).await?;

// let sss = vector_to_json(&users)?;
// println!("{}", sss);

// // Dropped connection will go to the pool
// drop(conn);

// // The Pool must be disconnected explicitly because
// // it's an asynchronous operation.
// pool.disconnect().await?;


Ok((conn, pool))
} 