use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError};
use serde::{Deserialize, Serialize};
use serde_json;
mod utilities;
mod models;
mod db;
// use utilities::utils::vector_to_json;

#[allow(unused_imports, unused_variables)]
#[tokio::main]
async fn main()  -> Result<(), mysql_async::Error> {
let mut pool = db::connection().await?;
let query = "SELECT ID, signature FROM users";
let result = db::execute_query(query, &pool).await;


 Ok(())
} 

