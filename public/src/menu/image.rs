use axum::{body::Bytes, extract::Path};

use crate::error::ApiResult;

const IDLI: &[u8] = include_bytes!("test_images/idli.jpg");
const KADAPA_GHEE_KARAM_DOSA: &[u8] = include_bytes!("test_images/Kadapa Ghee Karam Dosa.jpg");
const KADAPA_KARAM_DOSA: &[u8] = include_bytes!("test_images/Kadapa Karam Dosa.jpg");
const MASALA_DOSA: &[u8] = include_bytes!("test_images/Masala Dosa.png");
const ONION_DOSA: &[u8] = include_bytes!("test_images/Onion Dosa.jpg");
const ONION_MASALA_DOSA: &[u8] = include_bytes!("test_images/Onion Masala Dosa.jpg");
const PLAIN_DOSA: &[u8] = include_bytes!("test_images/Plain Dosa.png");

/// Get the full resolution image
#[worker::send]
#[axum::debug_handler]
pub async fn get_image(Path(id): Path<usize>) -> ApiResult<Bytes> {
    Ok(Bytes::from(match id {
        2 => PLAIN_DOSA,
        3 => ONION_DOSA,
        4 => MASALA_DOSA,
        5 => ONION_MASALA_DOSA,
        6 => KADAPA_KARAM_DOSA,
        7 => KADAPA_GHEE_KARAM_DOSA,
        _ => IDLI,
    }))
}

/// Get the 100x100 resolution image
#[worker::send]
#[axum::debug_handler]
pub async fn get_thumbnail(Path(id): Path<usize>) -> ApiResult<Bytes> {
    Ok(Bytes::from(match id {
        2 => PLAIN_DOSA,
        3 => ONION_DOSA,
        4 => MASALA_DOSA,
        5 => ONION_MASALA_DOSA,
        6 => KADAPA_KARAM_DOSA,
        7 => KADAPA_GHEE_KARAM_DOSA,
        _ => IDLI,
    }))
}
