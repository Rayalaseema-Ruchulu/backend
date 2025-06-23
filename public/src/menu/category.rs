use models::basic_details::BasicItemDetails;
use worker::{Request, Response, Result, RouteContext, query};

/// Get basic info for items in a category
pub(crate) async fn get_category(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
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
