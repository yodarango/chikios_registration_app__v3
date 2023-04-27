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
let pool = db::connection().await?;
let result = db::queries::users::get_all(pool).await?;

for res in result {
    println!("{:?}", res);
}
 Ok(())
} 

