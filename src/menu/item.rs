use models::{
    basic_details::BasicItemDetails, full_details::FullItemDetails,
    ingredient_or_category::IngredientOrCategory, item_details::ItemDetails,
};
use worker::{Request, Response, Result, RouteContext, query};

pub(crate) async fn get_item(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.param("id") {
        Some(id) => match id.parse::<usize>() {
            Ok(id) => {
                let db = ctx.d1("menu-db")?;

                let queries = db.batch(vec![
                    query!(
                        &ctx.d1("menu-db")?,
                        "SELECT * FROM menu_items WHERE id = ?",
                        &id
                    )?,
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

                let basic_details = queries[0].results::<BasicItemDetails>()?;
                let ingredients = queries[1].results::<IngredientOrCategory>()?;
                let categories = queries[2].results::<IngredientOrCategory>()?;

                Response::builder().from_json(&FullItemDetails {
                    basic_details: basic_details[0].clone(),
                    item_details: ItemDetails {
                        ingredients,
                        categories,
                    },
                })
            }
            Err(_) => Ok(Response::builder().with_status(442).empty()),
        },
        None => Ok(Response::builder().with_status(400).empty()),
    }
}
