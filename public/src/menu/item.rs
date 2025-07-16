use axum::{
    Json,
    extract::{Path, State},
};
use models::{
    basic_details::BasicItemDetails, full_details::FullItemDetails, item_details::ItemDetails,
};
use worker::{Env, query};

use crate::error::ApiResult;

/// Get the full details of an item
#[worker::send]
#[axum::debug_handler]
pub(crate) async fn get_item(
    State(env): State<Env>,
    Path(id): Path<usize>,
) -> ApiResult<Json<FullItemDetails>> {
    let db = env.d1("menu-db")?;

    let queries = db.batch(vec![
                    query!(
                        &db,
                        "SELECT * FROM menu_items WHERE id = ?",
                        &id
                    )?,
                    query!(
                        &db,
                        "SELECT ingredient_id, ingredients.name FROM item_ingredients INNER JOIN ingredients ON item_ingredients.ingredient_id = ingredients.id WHERE menu_item_id = ?",
                        &id
                    )?,
                    query!(
                        &db,
                        "SELECT category_id, categories.name FROM item_categories INNER JOIN categories ON item_categories.category_id = categories.id WHERE category_id = ?",
                        &id
                    )?,
                ]).await?;

    let basic_details = queries[0].results::<BasicItemDetails>()?;
    let ingredients = queries[1].results::<usize>()?;
    let categories = queries[2].results::<usize>()?;

    Ok(Json(FullItemDetails {
        basic_details: basic_details[0].clone(),
        item_details: ItemDetails {
            ingredients,
            categories,
        },
    }))
}
