use models::{id::Id, item_details::ItemDetails};
use worker::{Request, Response, Result, RouteContext, query};

pub(crate) async fn get_details(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.param("id") {
        Some(id) => match id.parse::<usize>() {
            Ok(id) => {
                let db = ctx.d1("menu-db")?;

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

                Response::builder().from_json(&ItemDetails {
                    ingredients,
                    categories,
                })
            }
            Err(_) => Ok(Response::builder().with_status(442).empty()),
        },
        None => Ok(Response::builder().with_status(400).empty()),
    }
}
