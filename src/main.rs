use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError};
use serde::{Deserialize, Serialize};
use serde_json;
mod models;
mod utilities;
mod db;
// use utilities::utils::vector_to_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    ID: u64,
    signature: String,
}

#[tokio::main]
async fn main()  -> Result<(), mysql_async::Error> {
// let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock")?;
// let pool = Pool::new(db_url);
// let mut conn = pool.get_conn().await?;
let (conn, pool) = db::connection().await?;

let users = "SELECT ID, signature FROM users"
.with(())
.map(conn, |(ID, signature)| User { ID, signature }).await?;

for user in &users {
    println!("user: {:?}", user);
}

// let sss = vector_to_json(&users)?;
// println!("{}", sss);

// Dropped connection will go to the pool
// drop(conn);

// The Pool must be disconnected explicitly because
// it's an asynchronous operation.
pool.disconnect().await?;


 Ok(())
} 

