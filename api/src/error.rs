use axum::http::StatusCode;
use axum_thiserror::ErrorStatus;
use deadpool_diesel::{InteractError, PoolError};
use thiserror::Error;

#[derive(Debug, Error, ErrorStatus)]
pub enum AppError {
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    Pg(#[from] PoolError),
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DeadPool(#[from] InteractError),
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    Diesel(#[from] diesel::result::Error),
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    Doli(#[from] doli_client_api_rs::error::DoliApiClientError),
    #[error("Misconfigured cover API on server side")]
    #[status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)]
    _Conf,
    #[error("Discount code invalid")]
    #[status(axum::http::StatusCode::BAD_REQUEST)]
    DiscountClient(#[from] discounts_client::error::DiscountClientError),
    #[error("Discount code period time invalid")]
    #[status(axum::http::StatusCode::BAD_REQUEST)]
    DiscountPeriod,
    #[error("User id does not exist")]
    #[status(axum::http::StatusCode::BAD_REQUEST)]
    _InexistentId,
}
