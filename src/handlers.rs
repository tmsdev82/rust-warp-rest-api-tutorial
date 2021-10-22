use crate::{models, ItemsDb, Result};
use warp::{http::StatusCode, reply, Reply};

pub async fn shopping_list_item_post_handler(
    mut shopping_list_item: models::ShoppingListItem,
    items_db: ItemsDb,
) -> Result<impl Reply> {
    let mut local_db = items_db.lock().await;
    let item_count = local_db.keys().len();
    shopping_list_item.item_id = Some(item_count);
    local_db.insert(item_count, shopping_list_item.clone());
    let local_db: Vec<models::ShoppingListItem> = local_db.values().cloned().collect();
    println!("Received UserData: {:?}", shopping_list_item);

    Ok(reply::with_status(
        reply::json(&local_db),
        StatusCode::CREATED,
    ))
}
