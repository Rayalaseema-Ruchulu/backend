use axum::{
    body::Bytes,
    extract::{Path, State},
};
use worker::Env;

use crate::error::ApiResult;

// const IDLI: &[u8] = include_bytes!("test_images/idli.jpg");
// const KADAPA_GHEE_KARAM_DOSA: &[u8] = include_bytes!("test_images/Kadapa Ghee Karam Dosa.jpg");
// const KADAPA_KARAM_DOSA: &[u8] = include_bytes!("test_images/Kadapa Karam Dosa.jpg");
// const MASALA_DOSA: &[u8] = include_bytes!("test_images/Masala Dosa.png");
// const ONION_DOSA: &[u8] = include_bytes!("test_images/Onion Dosa.jpg");
// const ONION_MASALA_DOSA: &[u8] = include_bytes!("test_images/Onion Masala Dosa.jpg");
// const PLAIN_DOSA: &[u8] = include_bytes!("test_images/Plain Dosa.png");

/// Get the full resolution image
#[worker::send]
#[axum::debug_handler]
pub async fn get_image(State(env): State<Env>, Path(id): Path<usize>) -> ApiResult<Bytes> {
    // TODO: Properly get image
    Ok(Bytes::from(
        env.kv("images-temp")?
            .get(&id.to_string())
            .bytes()
            .await
            .unwrap()
            .unwrap(),
    ))
}

/// Get the 100x100 resolution image
#[worker::send]
#[axum::debug_handler]
pub async fn get_thumbnail(State(env): State<Env>, Path(id): Path<usize>) -> ApiResult<Bytes> {
    // Temporarily add every image (so that image storage doesn't cost anything)
    // env.kv("images-temp")?.put_bytes("1", IDLI).unwrap().execute().await.unwrap();
    // env.kv("images-temp")?.put_bytes("2", PLAIN_DOSA).unwrap().execute().await.unwrap();
    // env.kv("images-temp")?.put_bytes("3", ONION_DOSA).unwrap().execute().await.unwrap();
    // env.kv("images-temp")?.put_bytes("4", MASALA_DOSA).unwrap().execute().await.unwrap();
    // env.kv("images-temp")?.put_bytes("5", ONION_MASALA_DOSA).unwrap().execute().await.unwrap();
    // env.kv("images-temp")?.put_bytes("6", KADAPA_KARAM_DOSA).unwrap().execute().await.unwrap();
    // env.kv("images-temp")?.put_bytes("7", KADAPA_GHEE_KARAM_DOSA).unwrap().execute().await.unwrap();

    Ok(Bytes::from(
        env.kv("images-temp")?
            .get(&id.to_string())
            .bytes()
            .await
            .unwrap()
            .unwrap(),
    ))
}
