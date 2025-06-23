use models::{ingredient_or_category::IngredientOrCategory, item_details::ItemDetails};
use worker::{Request, Response, Result, RouteContext, query};

pub(crate) async fn get_details(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.param("id") {
        Some(id) => match id.parse::<usize>() {
            Ok(id) => {
                let db = ctx.d1("menu-db")?;

                let queries = db.batch(vec![
                    query!(
                        &ctx.d1("menu-db")?,
                        "SELECT ingredient_id, ingredients.name FROM item_ingredients INNER JOIN ingredients ON item_ingredients.ingredient_id = ingredients.id WHERE menu_item_id = ?",
                        &id
                    )?,
                    query!(
                        &ctx.d1("menu-db")?,
                        "SELECT category_id, categories.name FROM item_categories INNER JOIN categories ON item_categories.category_id = categories.id WHERE category_id = ?",
                        &id
                    )?,
                ]).await?;

                let ingredients = queries[0].results::<IngredientOrCategory>()?;
                let categories = queries[1].results::<IngredientOrCategory>()?;

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
