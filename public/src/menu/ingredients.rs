use axum::{
    Json,
    extract::{Path, State},
};
use models::ingredient_or_category::IngredientOrCategory;
use worker::{Env, query};

use crate::error::{ApiError, ApiResult};

/// Gets all the ingredients
#[worker::send]
#[axum::debug_handler]
pub(crate) async fn get_ingredients(
    State(env): State<Env>,
) -> ApiResult<Json<Vec<IngredientOrCategory>>> {
    let db = env.d1("menu-db")?;
    let results = query!(&db, "SELECT * FROM ingredients")
        .all()
        .await?
        .results::<IngredientOrCategory>()?;

    Ok(Json(results))
}

/// Gets the name of an ingredient
#[worker::send]
#[axum::debug_handler]
pub(crate) async fn get_ingredient(
    State(env): State<Env>,
    Path(id): Path<usize>,
) -> ApiResult<Json<IngredientOrCategory>> {
    let db = env.d1("menu-db")?;
    let results = query!(&db, "SELECT * FROM ingredients WHERE id = ?", id)?
        .first::<IngredientOrCategory>(None)
        .await?;

    match results {
        Some(ingredient) => Ok(Json(ingredient)),
        None => Err(ApiError::NotFound),
    }
}
