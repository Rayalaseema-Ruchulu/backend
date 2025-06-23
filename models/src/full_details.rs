use serde::{Deserialize, Serialize};

use crate::{basic_details::BasicItemDetails, item_details::ItemDetails};

#[derive(Deserialize, Serialize)]
pub struct FullItemDetails {
    #[serde(flatten)]
    pub basic_details: BasicItemDetails,
    #[serde(flatten)]
    pub item_details: ItemDetails,
}
