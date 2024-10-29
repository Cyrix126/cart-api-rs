use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cart {
    pub lines: Vec<CartLine>,
    pub discount_code: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct CartLine {
    pub product_id: u32,
    pub qty: u32,
}
