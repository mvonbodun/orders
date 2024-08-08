use log::{debug, error};

use crate::model::{Order, OrderBuilder, OrderCreateRequest};
use crate::persistence::orders_dao::OrdersDao;

pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InternalError("An unexpected error occurred. Please try again".to_owned())
    }
}

pub async fn create_order(
    order_create_request: OrderCreateRequest,
    orders_dao: &(dyn OrdersDao + Sync + Send),
) -> Result<Order, HandlerError> {
    debug!("Before call to create_order hander_inner");
    let mut order = OrderBuilder::new();
    if order_create_request.order_ref.is_some() {
        order.order_ref(order_create_request.order_ref.unwrap());
    };
    if order_create_request.bill_to.is_some() {
        order.bill_to(order_create_request.bill_to.unwrap());
    };
    if order_create_request.order_items.is_some() {
        order.order_items(order_create_request.order_items.unwrap());
    };

    let result = orders_dao.create_order(order.build()).await;

    match result {
        Ok(order) => Ok(order),
        Err(e) => {
            error!("Error creating order: {}", e);
            Err(HandlerError::InternalError(format!(
                "Failed to create order: {}",
                e
            )))
        }
    }
}

pub async fn get_order(
    order_id: String,
    orders_dao: &(dyn OrdersDao + Sync + Send),
) -> Result<Option<Order>, HandlerError> {
    debug!("Before call to get_order hander_inner");
    let result = orders_dao.get_order(order_id).await;
    debug!("After call to get_order hander_inner: {:?}", result);

    match result {
        Ok(Some(order)) => Ok(Some(order)),
        Ok(None) => Ok(None),
        Err(e) => {
            error!("Error getting order: {}", e);
            Err(HandlerError::InternalError(format!(
                "Failed to get order: {}",
                e
            )))
        }
    }
}

pub async fn delete_order(
    order_id: String,
    orders_dao: &(dyn OrdersDao + Send + Sync),
) -> Result<(), HandlerError> {
    let result = orders_dao.delete_order(order_id).await;

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Error deleting order: {}", e);
            Err(HandlerError::default_internal_error())
        }
    }
}
