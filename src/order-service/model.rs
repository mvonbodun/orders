use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderTotals {
    pub product_total: f32,
    pub tax_total: Option<f32>,
    pub tax_detail: Option<HashMap<String, f32>>,
    pub shipping_total: Option<f32>,
    pub shipping_detail: Option<HashMap<String, f32>>,
    pub discount_total: Option<f32>,
    pub discount_detail: Option<HashMap<String, f32>>,
}

impl OrderTotals {
    pub fn builder() -> OrderTotalsBuilder {
        OrderTotalsBuilder::default()
    }
}

#[derive(Default)]
pub struct OrderTotalsBuilder {
    product_total: f32,
    tax_total: Option<f32>,
    tax_detail: Option<HashMap<String, f32>>,
    shipping_total: Option<f32>,
    shipping_detail: Option<HashMap<String, f32>>,
    discount_total: Option<f32>,
    discount_detail: Option<HashMap<String, f32>>,
}

impl OrderTotalsBuilder {
    pub fn new(product_total: f32) -> OrderTotalsBuilder {
        OrderTotalsBuilder {
            product_total,
            tax_total: None,
            tax_detail: None,
            shipping_total: None,
            shipping_detail: None,
            discount_total: None,
            discount_detail: None,
        }
    }

    pub fn tax_total(&mut self, tax_total: f32) -> &mut Self {
        self.tax_total = Some(tax_total);
        self
    }
    pub fn tax_detail(&mut self, tax_detail: HashMap<String, f32>) -> &mut Self {
        self.tax_detail = Some(tax_detail);
        self
    }
    pub fn shipping_total(&mut self, shipping_total: f32) -> &mut Self {
        self.shipping_total = Some(shipping_total);
        self
    }
    pub fn shipping_detail(&mut self, shipping_detail: HashMap<String, f32>) -> &mut Self {
        self.shipping_detail = Some(shipping_detail);
        self
    }
    pub fn discount_total(&mut self, discount_total: f32) -> &mut Self {
        self.discount_total = Some(discount_total);
        self
    }
    pub fn discount_detail(&mut self, discount_detail: HashMap<String, f32>) -> &mut Self {
        self.discount_detail = Some(discount_detail);
        self
    }
    pub fn build(&mut self) -> OrderTotals {
        OrderTotals {
            product_total: self.product_total,
            tax_total: self.tax_total,
            tax_detail: self.tax_detail.clone(),
            shipping_total: self.shipping_total,
            shipping_detail: self.shipping_detail.clone(),
            discount_total: self.discount_total,
            discount_detail: self.discount_detail.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderCreateRequest {
    pub order_ref: Option<String>,
    pub sold_to: Option<Address>,
    pub order_items: Option<Vec<OrderItem>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub order_ref: Option<String>,
    pub sold_to: Option<Address>,
    pub bill_to: Option<Address>,
    pub order_items: Option<Vec<OrderItem>>,
    pub order_totals: Option<OrderTotals>,
}
impl Order {
    pub fn builder() -> OrderBuilder {
        OrderBuilder::default()
    }
}

#[derive(Default)]
pub struct OrderBuilder {
    id: String,
    order_ref: Option<String>,
    sold_to: Option<Address>,
    bill_to: Option<Address>,
    order_items: Option<Vec<OrderItem>>,
    order_totals: Option<OrderTotals>,
}

impl OrderBuilder {
    pub fn new() -> OrderBuilder {
        OrderBuilder {
            id: Uuid::new_v4().to_string(),
            order_ref: None,
            sold_to: None,
            bill_to: None,
            order_items: None,
            order_totals: None,
        }
    }
    pub fn order_ref(&mut self, order_ref: String) -> &mut Self {
        self.order_ref = Some(order_ref);
        self
    }
    pub fn sold_to(&mut self, sold_to: Address) -> &mut Self {
        self.sold_to = Some(sold_to);
        self
    }
    pub fn bill_to(&mut self, bill_to: Address) -> &mut Self {
        self.bill_to = Some(bill_to);
        self
    }
    pub fn order_items(&mut self, order_items: Vec<OrderItem>) -> &mut Self {
        self.order_items = Some(order_items);
        self
    }
    pub fn order_totals(&mut self, order_totals: OrderTotals) -> &mut Self {
        self.order_totals = Some(order_totals);
        self
    }
    pub fn build(&mut self) -> Order {
        Order {
            id: Some(self.id.clone()),
            order_ref: self.order_ref.clone(),
            sold_to: self.sold_to.clone(),
            bill_to: self.bill_to.clone(),
            order_items: self.order_items.clone(),
            order_totals: self.order_totals.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub seq: Option<i32>,
    pub attribute_ref: Option<String>,
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn builder() -> AttributeBuilder {
        AttributeBuilder::default()
    }
}

#[derive(Default)]
pub struct AttributeBuilder {
    seq: Option<i32>,
    attribute_ref: Option<String>,
    name: String,
    value: String,
}

impl AttributeBuilder {
    pub fn new(name: String, value: String) -> AttributeBuilder {
        AttributeBuilder {
            seq: None,
            attribute_ref: None,
            name,
            value,
        }
    }
    pub fn seq(&mut self, seq: i32) -> &mut Self {
        self.seq = Some(seq);
        self
    }
    pub fn attribute_ref(&mut self, attribute_ref: String) -> &mut Self {
        self.attribute_ref = Some(attribute_ref);
        self
    }
    pub fn build(&mut self) -> Attribute {
        Attribute {
            seq: self.seq,
            attribute_ref: self.attribute_ref.clone(),
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Item {
    pub id: Option<String>,
    pub item_ref: String,
    pub product_id: Option<String>,
    pub product_ref: Option<String>,
    pub name: String,
    pub image_url: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub product_display_url: Option<String>,
}

impl Item {
    pub fn builder() -> ItemBuilder {
        ItemBuilder::default()
    }
}

#[derive(Default)]
pub struct ItemBuilder {
    id: Option<String>,
    item_ref: String,
    product_id: Option<String>,
    product_ref: Option<String>,
    name: String,
    image_url: Option<String>,
    attributes: Option<Vec<Attribute>>,
    product_display_url: Option<String>,
}

impl ItemBuilder {
    pub fn new(item_ref: String, name: String) -> ItemBuilder {
        ItemBuilder {
            id: None,
            item_ref,
            product_id: None,
            product_ref: None,
            name,
            image_url: None,
            attributes: None,
            product_display_url: None,
        }
    }
    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }
    pub fn product_id(&mut self, product_id: String) -> &mut Self {
        self.product_id = Some(product_id);
        self
    }
    pub fn product_ref(&mut self, product_ref: String) -> &mut Self {
        self.product_ref = Some(product_ref);
        self
    }
    pub fn image_url(&mut self, image_url: String) -> &mut Self {
        self.image_url = Some(image_url);
        self
    }
    pub fn attributes(&mut self, attributes: Vec<Attribute>) -> &mut Self {
        self.attributes = Some(attributes);
        self
    }
    pub fn product_display_url(&mut self, product_display_url: String) -> &mut Self {
        self.product_display_url = Some(product_display_url);
        self
    }
    pub fn build(&mut self) -> Item {
        Item {
            id: self.id.clone(),
            item_ref: self.item_ref.clone(),
            product_id: self.product_id.clone(),
            product_ref: self.product_ref.clone(),
            name: self.name.clone(),
            image_url: self.image_url.clone(),
            attributes: self.attributes.clone(),
            product_display_url: self.product_display_url.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub id: String,
    pub customer_ref: Option<String>,
    pub name: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub company: Option<String>,
    pub city: String,
    pub state_province: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub telephone: String,
    pub email: Option<String>,
}

#[derive(Default)]
pub struct AddressBuilder {
    id: String,
    customer_ref: Option<String>,
    name: String,
    address_line1: String,
    address_line2: Option<String>,
    company: Option<String>,
    city: String,
    state_province: Option<String>,
    postal_code: String,
    country: String,
    telephone: String,
    email: Option<String>,
}

impl Address {
    pub fn builder() -> AddressBuilder {
        AddressBuilder::default()
    }
}

impl AddressBuilder {
    pub fn new(
        id: String,
        name: String,
        address_line1: String,
        city: String,
        postal_code: String,
        country: String,
        telephone: String,
    ) -> AddressBuilder {
        AddressBuilder {
            id,
            customer_ref: None,
            name,
            address_line1,
            address_line2: None,
            company: None,
            city,
            state_province: None,
            postal_code,
            country,
            telephone,
            email: None,
        }
    }

    pub fn customer_ref(&mut self, customer_ref: String) -> &mut Self {
        self.customer_ref = Some(customer_ref);
        self
    }

    pub fn address_line2(&mut self, address_line2: String) -> &mut Self {
        self.address_line2 = Some(address_line2);
        self
    }

    pub fn company(&mut self, company: String) -> &mut Self {
        self.company = Some(company);
        self
    }

    pub fn state_province(&mut self, state_province: String) -> &mut Self {
        self.state_province = Some(state_province);
        self
    }

    pub fn email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    pub fn build(&mut self) -> Address {
        Address {
            id: self.id.clone(),
            customer_ref: self.customer_ref.clone(),
            name: self.name.clone(),
            address_line1: self.address_line1.clone(),
            address_line2: self.address_line2.clone(),
            company: self.company.clone(),
            city: self.city.clone(),
            state_province: self.state_province.clone(),
            country: self.country.clone(),
            postal_code: self.postal_code.clone(),
            telephone: self.telephone.clone(),
            email: self.email.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderItem {
    pub line_num: i32,
    pub order_id: String,
    pub quantity: i32,
    pub item: Item,
    pub price: Price,
    pub ship_to: Option<Address>,
    pub orderitem_totals: Option<OrderTotals>,
}

impl OrderItem {
    pub fn builder() -> OrderItemBuilder {
        OrderItemBuilder::default()
    }
}

#[derive(Default)]
pub struct OrderItemBuilder {
    line_num: i32,
    order_id: String,
    quantity: i32,
    item: Item,
    price: Price,
    ship_to: Option<Address>,
    orderitem_totals: Option<OrderTotals>,
}

impl OrderItemBuilder {
    pub fn new(
        line_num: i32,
        order_id: String,
        item: Item,
        quantity: i32,
        price: Price,
    ) -> OrderItemBuilder {
        OrderItemBuilder {
            line_num,
            order_id,
            item,
            price,
            quantity,
            ship_to: None,
            orderitem_totals: None,
        }
    }

    pub fn ship_to(&mut self, ship_to: Address) -> &mut Self {
        self.ship_to = Some(ship_to);
        self
    }

    pub fn orderitem_totals(&mut self, orderitem_totals: OrderTotals) -> &mut Self {
        self.orderitem_totals = Some(orderitem_totals);
        self
    }

    pub fn build(&mut self) -> OrderItem {
        OrderItem {
            line_num: self.line_num,
            order_id: self.order_id.clone(),
            quantity: self.quantity,
            item: self.item.clone(),
            price: self.price.clone(),
            orderitem_totals: self.orderitem_totals.clone(),
            ship_to: self.ship_to.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Price {
    pub id: Option<String>,
    pub amount: f64,
    pub currency: String,
}

impl Price {
    pub fn new(amount: f64, currency: String) -> Price {
        Price {
            id: None,
            amount,
            currency,
        }
    }
}

// impl Display for Price {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} {}", self.amount, self.currency::from_num)
//     }
// }

#[derive(Debug, Error)]
pub enum DBError {
    #[error("Database connection error")]
    Connection,
    #[error("Database query error")]
    Query,
    #[error("Database transaction error")]
    Transaction,
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_order() {
        // basic order
        let mut order = OrderBuilder::new();

        order
            .bill_to(
                AddressBuilder::new(
                    1.to_string(),
                    String::from("Michael VB"),
                    String::from("67 Heritage Hill Cir"),
                    String::from("The Woodlands"),
                    String::from("77381"),
                    String::from("USA"),
                    String::from("703-662-1407"),
                )
                .build(),
            )
            .order_items(vec![
                OrderItemBuilder::new(
                    1,
                    1.to_string(),
                    ItemBuilder::new("4423TWER".to_string(), "Nike Vapor Fly".to_string()).build(),
                    2,
                    Price::new(2.99, "USD".to_string()),
                )
                .ship_to(
                    AddressBuilder::new(
                        1.to_string(),
                        String::from("Michael VB"),
                        String::from("67 Heritage Hill Cir"),
                        String::from("The Woodlands"),
                        String::from("77381"),
                        String::from("USA"),
                        String::from("444-555-6666"),
                    )
                    .build(),
                )
                .orderitem_totals(
                    OrderTotalsBuilder::new(6.99)
                        .tax_total(0.43)
                        .shipping_total(2.99)
                        .build(),
                )
                .build(),
                OrderItemBuilder::new(
                    2,
                    1.to_string(),
                    ItemBuilder::new("KGHJ&%^&".to_string(), "Saucony Ride".to_string()).build(),
                    2,
                    Price::new(10.99, "USD".to_string()),
                )
                .ship_to(
                    AddressBuilder::new(
                        1.to_string(),
                        String::from("Michael VB"),
                        String::from("67 Heritage Hill Cir"),
                        String::from("The Woodlands"),
                        String::from("77381"),
                        String::from("USA"),
                        String::from("444-555-6666"),
                    )
                    .build(),
                )
                .orderitem_totals(
                    OrderTotalsBuilder::new(10.99)
                        .tax_total(0.43)
                        .shipping_total(2.99)
                        .build(),
                )
                .build(),
            ])
            .order_totals(
                OrderTotalsBuilder::new(12.99)
                    .tax_total(1.43)
                    .shipping_total(4.99)
                    .build(),
            )
            .build();
        println!("{:#?}", order.build());

        let order_json = serde_json::to_string(&order.build()).unwrap();
        println!("{:#?}", order_json);
    }
}
