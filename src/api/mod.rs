pub mod get_requests;

use warp::{Filter, Reply};

pub async fn server () {
    
    let routes = warp::path("api").map(|| "Server is running on port 303000")
    .with(warp::cors().allow_any_origin());

    warp::serve(x).run(([127, 0, 0, 1], 3030)).await;
}