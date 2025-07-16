use axum::{
    Json,
    extract::{Path, State},
};
use models::{id::Id, item_details::ItemDetails};
use worker::{Env, query};

use crate::error::ApiResult;

/// Retrieve the ingredients and categories of an item
#[worker::send]
#[axum::debug_handler]
pub(crate) async fn get_details(
    State(env): State<Env>,
    Path(id): Path<usize>,
) -> ApiResult<Json<ItemDetails>> {
    let db = env.d1("menu-db")?;

    let queries = db
        .batch(vec![
            // ingredients
            query!(
                &db,
                "SELECT ingredient_id FROM item_ingredients WHERE menu_item_id = ?",
                &id
            )?,
            // Categories
            query!(
                &db,
                "SELECT category_id FROM item_categories WHERE menu_item_id = ?",
                &id
            )?,
        ])
        .await?;

    let ingredients = queries[0].results::<Id>()?.iter().map(|id| id.id).collect();
    let categories = queries[1].results::<Id>()?.iter().map(|id| id.id).collect();

    Ok(Json(ItemDetails {
        ingredients,
        categories,
    }))
}
