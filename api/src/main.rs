mod config;
mod db;
mod error;
mod handler;
mod models;
mod schema;
use crate::handler::read_cart;
use axum::{
    routing::{get, post},
    serve, Router,
};
use config::Config;
use db::run_migrations;
use deadpool_diesel::postgres::Pool;
use get_pass::url::add_pass_to_url;
use handler::{create_cart, delete_cart, update_cart};
/// since the carts are private and only accessed by their respective users, there is no need to put them in cache.
#[derive(Clone)]
struct AppState {
    config: Config,
    pool: Pool,
    client_discount: discounts_client::Client,
    client_product: doli_client_api_rs::Client,
    client_customer: doli_client_api_rs::Client,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let config: Config = confy::load_path("/etc/name_api/config.toml")?;

    let pool = Pool::builder(deadpool_diesel::Manager::new(
        config.db_uri.as_str(),
        deadpool_diesel::Runtime::Tokio1,
    ))
    .build()?;
    run_migrations(&pool).await?;
    let state = construct_state(config, pool)?;
    let listener =
        tokio::net::TcpListener::bind(format!("127.0.0.1:{}", state.config.listen_port)).await?;
    serve(listener, router(state)).await?;
    Ok(())
}
/// API endpoint with :user must be protected so that only users and admin can access them
fn router(state: AppState) -> Router {
    Router::new()
        .route("/customer/:user/cart", post(create_cart))
        .route(
            "/customer/:user/cart/:cart",
            get(read_cart).put(update_cart).delete(delete_cart),
        )
        .with_state(state)
}

fn construct_state(config: Config, pool: Pool) -> Result<AppState, Box<dyn std::error::Error>> {
    let mut uri_product_api = config.product_api_uri.clone();
    add_pass_to_url(&mut uri_product_api, &config.product_api_pass_path)?;
    let client_product = doli_client_api_rs::Client::new(uri_product_api)?;

    let mut uri_discount_api = config.discount_api_uri.clone();
    add_pass_to_url(&mut uri_discount_api, &config.discount_api_pass_path)?;
    let client_discount = discounts_client::Client::new(uri_discount_api);

    Ok(AppState {
        config,
        pool,
        client_discount,
        client_customer: client_product.clone(),
        client_product,
    })
}
