use std::str::FromStr;

use log::{debug, error, info};

use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::Collection;

use crate::model::{DBError, Order};

#[async_trait]
pub trait OrdersDao {
    async fn create_order(&self, order: Order) -> Result<Order, DBError>;
    async fn delete_order(&self, order_id: String) -> Result<(), DBError>;
    async fn get_order(&self, order_id: String) -> Result<Option<Order>, DBError>;
}

pub struct OrdersDaoImpl {
    collection: Collection<Order>,
}

impl OrdersDaoImpl {
    pub fn new(collection: Collection<Order>) -> Self {
        OrdersDaoImpl { collection }
    }
}

#[async_trait]
impl OrdersDao for OrdersDaoImpl {
    // Create an Order
    async fn create_order(&self, order: Order) -> Result<Order, DBError> {
        // Implement logic to create an order in the database
        // and return the created order or an error if one occurred
        let insert_result = self.collection.insert_one(&order).await.map_err(|error| {
            error!("Error on insert: {:?}", error);
            DBError::Other(Box::new(error))
        })?;

        info!("Inserted order result: {:?}", insert_result);
        debug!("Order after insert: {:?}", order);
        Ok(order)
    }

    // Get an Order
    async fn get_order(&self, order_id: String) -> Result<Option<Order>, DBError> {
        // Implement logic to get an order from the database
        // by its order ID and return the order or an error if one occurred
        debug!("before call to find_one - order_id: {:?}", order_id);
        let find_result = self
            .collection
            .find_one(doc! {"_id": &order_id})
            .await
            .map_err(|error| {
                error!("DB error: {:?}", error);
                DBError::Other(Box::new(error))
            })?;

        match find_result {
            Some(order) => {
                debug!("Found order: {:?}", order);
                Ok(Some(order))
            }
            None => {
                debug!("Order not found for order_id: {:?}", order_id);
                Ok(None)
            }
        }
    }

    // Delete an Order
    async fn delete_order(&self, order_id: String) -> Result<(), DBError> {
        // Implement logic to delete an order from the database
        // by its order ID and return a success or an error if one occurred
        let delete_result = self
            .collection
            .delete_one(doc! {"_id": order_id})
            .await
            .map_err(|error| DBError::Other(Box::new(error)))?;

        debug!("Deleted order result: {:?}", delete_result);
        Ok(())
    }
}
