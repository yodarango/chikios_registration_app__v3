
use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError};
use crate::models::user::User;

pub async fn connection() -> Result<(&'static Conn, Pool), mysql_async::Error> {
let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock")?;
let pool = Pool::new(db_url);
let mut conn = pool.get_conn().await?;

// pass the connection and the pool to the caller
Ok((&mut conn, pool))
} 

pub async fn get_users(conn: &mut Conn, pool: &Pool) -> Result<Vec<User>, mysql_async::Error> {
    let users = conn
        .query_map(
            "SELECT ID, signature FROM users",
            |(ID, signature)| User { ID, signature },
        )
        .await?;

    Ok(users)
}

pub async fn execute_query () -> Result<(), mysql_async::Error> {
let (conn, pool) = connection().await?;

let users = "SELECT ID, signature FROM users"
.with(())
.map(conn, |(ID, signature)| User { ID, signature }).await?;

for user in &users {
    println!("user: {:?}", user);
}

// let sss = vector_to_json(&users)?;
// println!("{}", sss);

// Dropped connection will go to the pool
 drop(conn);

// The Pool must be disconnected explicitly because
// it's an asynchronous operation.
pool.disconnect().await?;
Ok(())
}

// // possible solution from ChatGpt
// use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError};
// use warp::{Filter, Rejection};

// async fn with_db(
//     conn: Conn,
//     pool: Pool,
//     req: warp::http::Request,
// ) -> Result<warp::http::Request, Rejection> {
//     Ok(req.insert_state((conn, pool)))
// }

// pub fn db_middleware() -> impl Filter<Extract = ((Conn, Pool,),), Error = Rejection> + Clone {
//     let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock").unwrap();
//     let pool = Pool::new(db_url);
    
//     warp::any()
//         .map(move || {
//             let pool = pool.clone();
//             async move {
//                 let conn = pool.get_conn().await.unwrap();
//                 (conn, pool)
//             }
//         })
//         .and_then(with_db)
// }

// let route = warp::path("example")
//     .and(db_middleware())
//     .map(|(conn, pool)| {
//         // Use `conn` and `pool` to handle the request
//     });
