mod menu;

use menu::category::get_category;
use menu::details::get_details;
use menu::item::get_item;
use worker::{Context, Env, Request, Response, Result, Router, event};

use crate::menu::categories::get_categories;
use crate::menu::ingredients::get_ingredients;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/menu/categories", get_categories)
        .get_async("/menu/category/:id", get_category)
        .get_async("/menu/ingredients", get_ingredients)
        .get_async("/menu/:id", get_item)
        .get_async("/menu/:id/details", get_details)
        .run(req, env)
        .await
}
