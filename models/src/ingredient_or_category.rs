use serde::{Deserialize, Serialize};

// Since ingredients and categories follow the same data format, we will
// use the same struct to store either one.
#[derive(Deserialize, Serialize)]
pub struct IngredientOrCategory {
    id: usize,
    name: String,
}
