use menu::new_item::post_new_item;
use worker::{Context, Env, Request, Response, Result, Router, event};

use crate::menu::service_bindings::send_to_common_backend;

mod menu;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/menu/categories", send_to_common_backend)
        .get_async("/menu/category/:id", send_to_common_backend)
        .get_async("/menu/ingredients", send_to_common_backend)
        .get_async("/menu/:id", send_to_common_backend)
        .get_async("/menu/:id/details", send_to_common_backend)
        .post_async("/menu/new", post_new_item)
        .run(req, env)
        .await
}
