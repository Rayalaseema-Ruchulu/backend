use serde::{Deserialize, Serialize};

use crate::ingredient_or_category::IngredientOrCategory;

#[derive(Deserialize, Serialize)]
pub struct ItemDetails {
    pub ingredients: Vec<IngredientOrCategory>,
    pub categories: Vec<IngredientOrCategory>,
}
