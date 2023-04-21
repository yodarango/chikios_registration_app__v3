use mysql_async::{prelude::*, Pool, Opts, UrlError};

struct User {
    ID: u64,
    signature: String,
}

#[tokio::main]
async fn main()  -> Result<(), mysql_async::Error> {
let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock")?;
let pool = Pool::new(db_url);
let mut conn = pool.get_conn().await?;

let users = "SELECT ID, signature FROM users"
.with(())
.map(&mut conn, |(ID, signature)| User { ID, signature }).await?;

for user in users {
    println!("ID: {} signature: {}", user.ID, user.signature);
}

// Dropped connection will go to the pool
drop(conn);

// The Pool must be disconnected explicitly because
// it's an asynchronous operation.
pool.disconnect().await?;


Ok(())
} 

