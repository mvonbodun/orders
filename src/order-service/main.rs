pub mod model;

use dotenvy::dotenv;
// use handlers::{create_order, delete_order, get_order};
use log::{debug, error, info, trace, warn};
use order_messages::{Address, OrderCreateRequest, OrderCreateResponse};
use persistence::orders_dao::{OrdersDao, OrdersDaoImpl};
use prost::Message;
use std::{env, error::Error, sync::Arc};
use tokio::signal;

use model::Order;
use mongodb::{Client, Collection};

use async_nats::service::ServiceExt;
use futures::StreamExt;

pub mod order_messages {
    include!(concat!(env!("OUT_DIR"), "/order_messages.rs"));
}

mod handlers;
mod persistence;

#[derive(Clone)]
pub struct AppState {
    pub orders_dao: Arc<dyn OrdersDao + Send + Sync>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let dir = env!("OUT_DIR");
    // initialize pretty_env_logger
    pretty_env_logger::init();
    info!("Test info logging");
    warn!("Test warn logging");
    error!("Test error loggin: {}", dir);
    trace!("test trace logging");
    debug!("test debug logging");

    // initialize dotenv
    dotenv().ok();

    // Get MongoDB URL
    let uri = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    // connect to MongoDB
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("db_orders");
    let orders_coll: Collection<Order> = database.collection("orders");

    let orders_dao = Arc::new(OrdersDaoImpl::new(orders_coll)).clone();

    // Connect to the nats server
    let client = async_nats::connect("0.0.0.0:4222").await?;

    let service = client
        .service_builder()
        .description("order service")
        .start("orders", "0.1.0")
        .await?;

    let g = service.group("orders");

    let mut create_order = g.endpoint("create").await?;

    debug!("Inside spawn order create service");
    tokio::spawn(async move {
        while let Some(request) = create_order.next().await {
            let order = OrderCreateRequest::decode(request.message.payload.clone());
            match order {
                Ok(order) => {
                    let mut model_addr: Option<model::Address> = None;
                    if order.sold_to.is_some() {
                        let req_addr = order.sold_to.unwrap();
                        let mut addr_bldr = model::AddressBuilder::new(
                            req_addr.id,
                            req_addr.name,
                            req_addr.address_line1,
                            req_addr.city,
                            req_addr.postal_code,
                            req_addr.country,
                            req_addr.telephone,
                        );
                        if req_addr.address_line2.is_some() {
                            addr_bldr.address_line2(req_addr.address_line2.unwrap());
                        }
                        if req_addr.company.is_some() {
                            addr_bldr.company(req_addr.company.unwrap());
                        }
                        if req_addr.state_province.is_some() {
                            addr_bldr.state_province(req_addr.state_province.unwrap());
                        }
                        if req_addr.email.is_some() {
                            addr_bldr.email(req_addr.email.unwrap());
                        }
                        model_addr = Some(addr_bldr.build());
                    }

                    let ocr = model::OrderCreateRequest {
                        order_ref: order.order_ref.clone(),
                        sold_to: model_addr,
                        order_items: None,
                    };
                    let result = handlers::create_order(orders_dao.clone(), ocr).await;
                    match result {
                        Ok(o) => {
                            let mut a: Option<Address> = None;
                            if o.sold_to.is_some() {
                                let db_addr = o.sold_to.unwrap();
                                let addr = Address {
                                    id: db_addr.id,
                                    customer_ref: db_addr.customer_ref,
                                    name: db_addr.name,
                                    address_line1: db_addr.address_line1,
                                    address_line2: db_addr.address_line2,
                                    company: db_addr.company,
                                    city: db_addr.city,
                                    state_province: db_addr.state_province,
                                    postal_code: db_addr.postal_code,
                                    country: db_addr.country,
                                    telephone: db_addr.telephone,
                                    email: db_addr.email,
                                };
                                a = Some(addr);
                            }

                            let ocrsp = OrderCreateResponse {
                                id: o.id.unwrap(),
                                order_ref: o.order_ref,
                                sold_to: a,
                            };

                            let mut buf = vec![];
                            ocrsp.encode(&mut buf).unwrap();
                            request.respond(Ok(buf.into())).await.unwrap();
                        }
                        Err(_) => {
                            request
                                .respond(Ok("error creating order".into()))
                                .await
                                .unwrap();
                        }
                    }
                }
                Err(err) => {
                    warn!("Invalid order format: {:?}", err);
                    request
                        .respond(Ok("Message could not be decoded".into()))
                        .await
                        .unwrap();
                }
            }
        }
    });

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Received SIGINT");
        }
        Err(err) => {
            error!("Error listening for SIGINT: {}", err);
        }
    }

    Ok(())
}

// async fn main() -> Result<(), Box<dyn Error>> {
//     // initialize pretty_env_logger
//     pretty_env_logger::init();
//     info!("Test info logging");
//     warn!("Test warn logging");
//     error!("Test error logging");
//     trace!("test trace logging");
//     debug!("test debug logging");

//     // initialize dotenv
//     dotenv().ok();

//     // Get MongoDB URL
//     let uri = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
//     // connect to MongoDB
//     let client = Client::with_uri_str(uri).await?;
//     let database = client.database("db_orders");
//     let orders_coll: Collection<Order> = database.collection("orders");

//     let orders_dao = Arc::new(OrdersDaoImpl::new(orders_coll)).clone();

//     let app_state = AppState { orders_dao };

//     let app = Router::new()
//         .route(
//             "/hello",
//             get(|| async { Html("hello <strong>World!!</strong>") }),
//         )
//         .route("/order", post(create_order))
//         .route("/order/:id", get(get_order))
//         .route("/order", delete(delete_order))
//         .with_state(app_state);

//     // Start Server
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
//         .await
//         .unwrap();
//     println!("->> LISTENING on {:?}\n", listener);
//     axum::serve(listener, app).await.unwrap();
//     Ok(())
// }
