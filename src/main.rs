pub mod model;

use dotenvy::dotenv;
use handlers::{create_order, delete_order, get_order};
use log::{debug, error, info, trace, warn};
use persistence::orders_dao::{OrdersDao, OrdersDaoImpl};
use std::{env, error::Error, sync::Arc};

use axum::{
    response::Html,
    routing::{delete, get, post},
    Router,
};
use model::Order;
use mongodb::{Client, Collection};

mod handlers;
mod persistence;

#[derive(Clone)]
pub struct AppState {
    pub orders_dao: Arc<dyn OrdersDao + Send + Sync>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize pretty_env_logger
    pretty_env_logger::init();
    info!("Test info logging");
    warn!("Test warn logging");
    error!("Test error logging");
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

    let app_state = AppState { orders_dao };

    let app = Router::new()
        .route(
            "/hello",
            get(|| async { Html("hello <strong>World!!</strong>") }),
        )
        .route("/order", post(create_order))
        .route("/order/:id", get(get_order))
        .route("/order", delete(delete_order))
        .with_state(app_state);

    // Start Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("->> LISTENING on {:?}\n", listener);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// if client.is_ok() {
//     let database = client.unwrap().database("sample_mflix");
//     let my_coll: Collection<Movie> = database.collection("movies");
//     let my_movie = my_coll
//         .find_one(doc! { "movie_uuid": "389af26b-8160-4910-b44f-24405a96d23d" })
//         .await;
//     if my_movie.is_ok() {
//         println!("Found a movie:\n:{:#?}", my_movie);
//     } else {
//         let e = my_movie.err();
//         println!("Error fetching movie: {:?}", e);
//     }
//     let uuid = Uuid::new_v4();
//     let movie = Movie {
//         id: ObjectId::new().to_string(),
//         movie_uuid: uuid.to_string(),
//         title: "The Perils of Pauline14".to_string(),
//         plot: Some("#14.".to_string()),
//         year: 1987,
//     };
//     println!("inserting movie: {:?}", movie);
//     let insert_result = my_coll.insert_one(movie).await?;
//     println!("Inserted movie: {:?}", insert_result.inserted_id);
// }
