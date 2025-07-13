use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ItemDetails {
    pub ingredients: Vec<usize>,
    pub categories: Vec<usize>,
}
