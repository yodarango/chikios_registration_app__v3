use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError};
use serde::{Deserialize, Serialize};
use serde_json;
mod utilities;
mod models;
mod api;
mod db;

use warp::{Filter, Rejection, Reply};
// use utilities::utils::vector_to_json;

#[allow(unused_imports, unused_variables)]
#[tokio::main]
async fn main()  -> Result<(), mysql_async::Error> {
    api::server().await;

 Ok(())
} 

