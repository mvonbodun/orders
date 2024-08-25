use clap::{Parser, Subcommand};
use order_messages::{Address, OrderCreateRequest, OrderCreateResponse};
use prost::Message;

pub mod order_messages {
    include!(concat!(env!("OUT_DIR"), "/order_messages.rs"));
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    OrderCreate {
        #[arg(short, long)]
        order_ref: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the nats server
    let client = async_nats::connect("0.0.0.0:4222").await?;

    let order = order_messages::OrderCreateRequest {
        order_ref: Some("234234".to_owned()),
        sold_to: Some(order_messages::Address {
            id: "123456".to_owned(),
            customer_ref: Some("asdfasdf".to_owned()),
            name: "John Doe".to_owned(),
            address_line1: "123 Main St".to_owned(),
            address_line2: None,
            company: None,
            city: "Anytown".to_owned(),
            state_province: Some("CA".to_owned()),
            postal_code: "90210".to_owned(),
            country: "USA".to_owned(),
            telephone: "123-456-7890".to_owned(),
            email: Some("john.doe@example.com".to_owned()),
        }),
    };

    let mut buf = vec![];
    order.encode(&mut buf)?;
    let result = client.request("orders.create", buf.into()).await?;
    let response = OrderCreateResponse::decode(result.payload)?;
    println!("response: {:?}", response);

    let order = order_messages::OrderCreateRequest {
        order_ref: None,
        sold_to: None,
    };

    let mut buf = vec![];
    order.encode(&mut buf)?;
    let result = client.request("orders.create", buf.into()).await?;
    let response = OrderCreateResponse::decode(result.payload)?;
    println!("response: {:?}", response);

    Ok(())
}
