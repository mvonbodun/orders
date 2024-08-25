use std::{error::Error, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use handlers_inner::HandlerError;

use crate::{
    model::{Order, OrderCreateRequest},
    persistence::orders_dao::OrdersDaoImpl,
    AppState,
};

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            handlers_inner::HandlerError::InternalError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}

// Create order
pub async fn create_order(
    orders_dao: Arc<OrdersDaoImpl>,
    order_create_request: OrderCreateRequest,
) -> Result<Order, HandlerError> {
    handlers_inner::create_order(order_create_request, orders_dao.as_ref()).await
}

// Get order
pub async fn get_order(
    State(AppState { orders_dao }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let result = handlers_inner::get_order(id, orders_dao.as_ref()).await;
    match result {
        Ok(Some(order)) => Ok(Json(order)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Order not found").into_response()),
        Err(e) => Err(e.into_response()),
    }
}

// Delete order
pub async fn delete_order(
    State(AppState { orders_dao }): State<AppState>,
    order_id: String,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::delete_order(order_id, orders_dao.as_ref())
        .await
        .map(Json)
}
