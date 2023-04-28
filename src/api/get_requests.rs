use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError};
use crate::models::user::User;
use crate::db;

pub async fn get_users()  -> Result<(), mysql_async::Error> {
let pool = db::connection().await?;
// let result = db::queries::users::get_all(pool).await?;

// for res in result {
//     println!("{:?}", res);
// }



//let pool = options::new();
    let conn = match pool.get_conn().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error getting connection to the database. Thi is the Error: {}", e);
            return Err(e);
        }
    };

    println!("{:#?}", conn);

 Ok(())
} 
