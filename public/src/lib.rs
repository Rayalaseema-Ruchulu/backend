mod cache;
mod error;
mod menu;

use axum::middleware;
use axum::{Router, routing::get};
use tower_service::Service;
use worker::{Context, Env, HttpRequest, Result, event};

use crate::cache::cache_control_middleware;
use crate::menu::category::*;
use crate::menu::details::get_details;
use crate::menu::image::*;
use crate::menu::ingredients::*;
use crate::menu::item::get_item;

pub(crate) type HttpResponse = axum::http::Response<axum::body::Body>;

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    console_error_panic_hook::set_once();

    Ok(Router::new()
        .route("/menu/categories", get(get_categories))
        .route("/menu/categories/featured", get(get_featured_categories))
        .route("/menu/category/{id}", get(get_category))
        .route("/menu/category/{id}/items", get(get_category_items))
        .route("/menu/ingredients", get(get_ingredients))
        .route("/menu/ingredient/{id}", get(get_ingredient))
        .route("/menu/{id}", get(get_item))
        .route("/menu/{id}/details", get(get_details))
        .route("/menu/{id}/image", get(get_image))
        .route("/menu/{id}/thumbnail", get(get_thumbnail))
        .layer(middleware::from_fn(cache_control_middleware))
        .with_state(env)
        .call(req)
        .await?)
}
