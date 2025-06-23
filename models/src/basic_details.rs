use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct BasicItemDetails {
    id: usize,
    name: String,
    description: String,
    price: f32,
}
