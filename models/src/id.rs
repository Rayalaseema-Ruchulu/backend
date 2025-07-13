use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    #[serde(alias = "ingredient_id", alias = "category_id")]
    pub id: usize,
}
