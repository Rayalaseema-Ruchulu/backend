use serde::{Deserialize, Serialize};

pub mod new_item;
pub mod service_bindings;
pub mod update_item;

#[derive(Serialize, Deserialize)]
struct NewItem {
    name: String,
    description: String,
    price: f32,
    categories: Vec<usize>,
    ingredients: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
struct UpdateItem {
    name: Option<String>,
    description: Option<String>,
    price: Option<f32>,
    categories: Option<Vec<usize>>,
    ingredients: Option<Vec<usize>>,
}
