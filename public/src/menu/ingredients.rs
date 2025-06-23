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
