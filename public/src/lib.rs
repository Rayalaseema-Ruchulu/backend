mod etag;
mod menu;

use menu::details::get_details;
use menu::item::get_item;
use worker::{Context, Env, Request, Response, Result, Router, event};

use crate::menu::category::{get_categories, get_category, get_category_items};
use crate::menu::image::{get_image, get_thumbnail};
use crate::menu::ingredients::{get_ingredient, get_ingredients};

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/menu/categories", get_categories)
        .get_async("/menu/category/:id", get_category)
        .get_async("/menu/category/:id/items", get_category_items)
        .get_async("/menu/ingredients", get_ingredients)
        .get_async("/menu/ingredient/:id", get_ingredient)
        .get_async("/menu/:id", get_item)
        .get_async("/menu/:id/details", get_details)
        .get_async("/menu/:id/image", get_image)
        .get_async("/menu/:id/thumbnail", get_thumbnail)
        .run(req, env)
        .await
}
