
use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError, Row};
use crate::models::user::User;

pub async fn connection() -> Result< Pool, mysql_async::Error> {
let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock")?;
let pool = Pool::new(db_url);

// pass the connection and the pool to the caller
Ok(pool)
} 

// pub async fn execute_query (query: &str, pool: &Pool) -> Result<(), mysql_async::Error> {

// let mut &conn = &pool.get_conn().await?;

// let users = query.with(())
// .map(&mut conn,  |(ID, signature)| User { ID, signature }).await?;

// for user in &users {
//     println!("user: {:?}", user);
// }

// // let sss = vector_to_json(&users)?;
// // println!("{}", sss);

// //Dropped connection will go to the pool
// drop(conn);

// // The Pool must be disconnected explicitly because
// // it's an asynchronous operation.
// pool.disconnect().await?;
// Ok(())
// }


pub async fn execute_query<T>(query: &str, pool: &Pool) -> Result<Vec<Row>, mysql_async::Error> 
    where T: for<'r> mysql_async::prelude<'r>
{
    let mut conn = pool.get_conn().await?;
    let result = conn.query::<T, _>(query).await?;
    drop(conn);
    pool.disconnect().await?;
    Ok(result)
}