use models::{basic_details::BasicItemDetails, ingredient_or_category::IngredientOrCategory};
use worker::{Request, Response, Result, RouteContext, query};

pub(crate) async fn get_categories(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.d1("menu-db")?;
    let results = query!(&db, "SELECT * FROM categories")
        .all()
        .await?
        .results::<IngredientOrCategory>()?;

    Response::builder().from_json(&results)
}

pub(crate) async fn get_category(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.param("id") {
        Some(id) => match id.parse::<usize>() {
            Ok(id) => {
                let db = ctx.d1("menu-db")?;
                let results = query!(&db, "SELECT * FROM categories WHERE id = ?", id)?
                    .first::<IngredientOrCategory>(None)
                    .await?;

                match results {
                    Some(category) => Ok(Response::builder().from_json(&category)?),
                    None => Ok(Response::builder().with_status(404).empty()),
                }
            }
            Err(_) => Ok(Response::builder().with_status(422).empty()),
        },
        None => Ok(Response::builder().with_status(400).empty()),
    }
}

/// Get basic info for items in a category
pub(crate) async fn get_category_items(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.param("id") {
        Some(id) => match id.parse::<usize>() {
            Ok(id) => {
                let query = query!(
                    &ctx.d1("menu-db")?,
                    "SELECT menu_items.* FROM item_categories INNER JOIN menu_items ON item_categories.menu_item_id = menu_items.id WHERE category_id = ?",
                    &id
                )?.all().await?.results::<BasicItemDetails>()?;

                Response::builder().from_json(&query)
            }
            Err(_) => Ok(Response::builder().with_status(442).empty()),
        },
        None => Ok(Response::builder().with_status(400).empty()),
    }
}
