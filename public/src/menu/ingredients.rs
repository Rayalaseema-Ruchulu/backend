use models::ingredient_or_category::IngredientOrCategory;
use worker::{Request, Response, Result, RouteContext, query};

pub(crate) async fn get_ingredients(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.d1("menu-db")?;
    let results = query!(&db, "SELECT * FROM ingredients")
        .all()
        .await?
        .results::<IngredientOrCategory>()?;

    Response::builder().from_json(&results)
}

pub(crate) async fn get_ingredient(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.param("id") {
        Some(id) => match id.parse::<usize>() {
            Ok(id) => {
                let db = ctx.d1("menu-db")?;
                let results = query!(&db, "SELECT * FROM ingredients WHERE id = ?", id)?
                    .first::<IngredientOrCategory>(None)
                    .await?;

                match results {
                    Some(ingredient) => Ok(Response::builder().from_json(&ingredient)?),
                    None => Ok(Response::builder().with_status(404).empty()),
                }
            }
            Err(_) => Ok(Response::builder().with_status(422).empty()),
        },
        None => Ok(Response::builder().with_status(400).empty()),
    }
}
