use warp::Filter;

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let routes = root.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}
