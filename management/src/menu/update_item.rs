use worker::{Request, Response, Result, RouteContext, query};

use crate::menu::UpdateItem;

pub async fn patch_update_item(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.param("id") {
        Some(id) => match id.parse::<usize>() {
            Ok(id) => {
                let body = req.json::<UpdateItem>().await?;
                let db = ctx.env.d1("menu-db")?;
                let mut queries = vec![];

                if let Some(name) = body.name {
                    queries.push(query!(
                        &db,
                        "UPDATE menu_items SET name = ? WHERE id = ?",
                        name,
                        id
                    )?);
                }

                if let Some(description) = body.description {
                    queries.push(query!(
                        &db,
                        "UPDATE menu_items SET description = ? WHERE id = ?",
                        description,
                        id
                    )?);
                }

                if let Some(price) = body.price {
                    queries.push(query!(
                        &db,
                        "UPDATE menu_items SET price = ? WHERE id = ?",
                        price,
                        id
                    )?);
                }

                if let Some(categories) = body.categories {
                    queries.push(query!(&db, ""));
                }

                if let Some(ingredients) = body.ingredients {
                    queries.push(query!(&db, ""));
                }

                todo!()
            }
            Err(_) => todo!(),
        },
        None => Ok(Response::builder().with_status(400).empty()),
    }
}
