use worker::{query, Error::RustError, Request, Response, Result, RouteContext};

use super::NewItem;

pub async fn post_new_item(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body = req.json::<NewItem>().await?;
    let db = ctx.env.d1("menu-d1")?;

    let insert_result_meta = query!(
        &db,
        "INSERT INTO menu_items (name, description, price) VALUES (?, ?, ?)",
        body.name,
        body.description,
        body.price
    )?
    .run()
    .await?
    .meta()?;

    let row_id = insert_result_meta
        .ok_or_else(|| {
            RustError("Insert metadata not found after creating menu item.".to_string())
        })?
        .last_row_id
        .ok_or_else(|| {
            RustError("last_row_id not found in metadata after creating menu item.".to_string())
        })?;

    let mut queries = vec![];

    for category in body.categories {
        queries.push(query!(
            &db,
            "INSERT INTO menu_item_categories (menu_item_id, category_id) VALUES (?, ?)",
            row_id,
            category
        )?);
    }
    for ingredient in body.ingredients {
        queries.push(query!(
            &db,
            "INSERT INTO menu_item_ingredients (menu_item_id, ingredient_id) VALUES (?, ?)",
            row_id,
            ingredient
        )?);
    }

    let batch_results = db.batch(queries).await?;

    let collected_errors = batch_results
        .iter()
        .filter_map(|item_result| item_result.error())
        .collect::<Vec<String>>();

    if collected_errors.is_empty() {
        Ok(Response::builder().with_status(201).empty())
    } else {
        let error_messages = collected_errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        Ok(Response::builder()
            .with_status(500)
            .from_json(&error_messages)?)
    }
}
