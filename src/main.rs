use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};

mod handlers;
mod models;

type ItemsDb = Arc<Mutex<HashMap<usize, models::ShoppingListItem>>>;
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let items_db: ItemsDb = Arc::new(Mutex::new(HashMap::new()));
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let shopping_list_items_route = warp::path("shopping_list_items")
        .and(warp::get())
        .and(with_items_db(items_db.clone()))
        .and_then(handlers::shopping_list_items_get_handler);

    let shopping_list_item_route = warp::path("shopping_list_item")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_items_db(items_db.clone()))
        .and_then(handlers::shopping_list_item_post_handler);
    let routes = root
        .or(shopping_list_items_route)
        .or(shopping_list_item_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}

fn with_items_db(
    items_db: ItemsDb,
) -> impl Filter<Extract = (ItemsDb,), Error = Infallible> + Clone {
    warp::any().map(move || items_db.clone())
}
