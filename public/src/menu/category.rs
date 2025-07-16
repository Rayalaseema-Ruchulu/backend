use axum::{
    Json,
    extract::{Path, State},
};
use models::{basic_details::BasicItemDetails, ingredient_or_category::IngredientOrCategory};
use worker::{Env, query};

use crate::error::{ApiError, ApiResult};

/// Retrieve all categories
#[worker::send]
#[axum::debug_handler]
pub(crate) async fn get_categories(
    State(env): State<Env>,
) -> ApiResult<Json<Vec<IngredientOrCategory>>> {
    let db = env.d1("menu-db")?;
    let results = query!(&db, "SELECT * FROM categories")
        .all()
        .await?
        .results::<IngredientOrCategory>()?;

    Ok(Json(results))
}

/// Get the name of a specific category with a category id
#[worker::send]
#[axum::debug_handler]
pub(crate) async fn get_category(
    State(env): State<Env>,
    Path(id): Path<usize>,
) -> ApiResult<Json<IngredientOrCategory>> {
    let db = env.d1("menu-db")?;
    let results = query!(&db, "SELECT * FROM categories WHERE id = ?", id)?
        .first::<IngredientOrCategory>(None)
        .await?;

    match results {
        Some(category) => Ok(Json(category)),
        None => Err(ApiError::NotFound),
    }
}

/// Get basic info for items in a category
#[worker::send]
#[axum::debug_handler]
pub(crate) async fn get_category_items(
    State(env): State<Env>,
    Path(id): Path<usize>,
) -> ApiResult<Json<Vec<BasicItemDetails>>> {
    let db = env.d1("menu-db")?;
    let results = query!(
        &db,
        "SELECT menu_items.* FROM item_categories INNER JOIN menu_items ON item_categories.menu_item_id = menu_items.id WHERE category_id = ?",
        &id
    )?
    .all()
    .await?
    .results::<BasicItemDetails>()?;

    Ok(Json(results))
}
