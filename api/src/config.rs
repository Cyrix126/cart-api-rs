use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    // cover database connection
    pub db_uri: Url,
    pub db_pass_path: PathBuf,
    // port on which the cover API will listen for incoming connections
    pub listen_port: u16,
    // cover database connection
    pub product_api_uri: Url,
    pub product_api_pass_path: PathBuf,
    // to update the cache when data is updated in db
    pub cache_api_uri: Url,
    pub cache_api_pass_path: PathBuf,
    // to check validity of discount code
    pub discount_api_uri: Url,
    pub discount_api_pass_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_uri: Url::parse("postgresql:://user@127.0.0.1:5432/mydb").unwrap(),
            db_pass_path: PathBuf::from("name_api/db/user"),
            listen_port: 10200,
            product_api_uri: Url::parse("https://ecommerce.com/product-api").unwrap(),
            product_api_pass_path: PathBuf::from("product-api/user"),
            cache_api_uri: Url::parse("https://ecommerce.com/cache-api").unwrap(),
            cache_api_pass_path: PathBuf::from("cache-api/user"),
            discount_api_uri: Url::parse("https://ecommerce.com/discount-api").unwrap(),
            discount_api_pass_path: PathBuf::from("discount-api/user"),
        }
    }
}
