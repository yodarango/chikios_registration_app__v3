pub mod users {
    use mysql_async::{prelude::*, Pool, Opts, Conn, UrlError, Row};
    use crate::models::user::User;

    pub async fn get_all(pool: Pool) -> Result<Vec<User>, mysql_async::Error> {

    let mut conn = pool.get_conn().await?;

    let users = "SELECT ID, signature FROM users".with(())
    .map(&mut conn,  |(ID, signature)| User { ID, signature }).await?;

    drop(conn);

    pool.disconnect().await?;
    Ok(users)
    }

}