use rocket::{response::content::RawHtml, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::db::{customers, purchaseOrders};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    id: Option<i64>,
    customer_id: Option<i64>,
    book_id: Option<i64>,
    shipped: Option<i64>,
}

#[post("/new", data = "<order>")]
pub fn create_order(order: Json<Order>) -> Result<(), String> {
    let cid = validate_id(order.customer_id.clone(), "cid")?;
    let bid = validate_id(order.book_id.clone(), "bid")?;

    purchaseOrders::create_purchase_order(cid, bid)?;
    Ok(())
}

#[get("/shipped", format = "json", data = "<order>")]
pub fn get_shipped(order: Json<Order>) -> Result<Json<Order>, String> {
    let cid = validate_id(order.customer_id.clone(), "cid")?;
    let bid = validate_id(order.book_id.clone(), "bid")?;

    let oid = purchaseOrders::get_purchase_order_id(cid, bid)?;
    let shipped = purchaseOrders::is_po_shipped(oid)?;
    Ok(Json(Order {
        id: None,
        customer_id: None,
        book_id: None,
        shipped: Some(shipped),
    }))
}

#[put("/ship", data = "<order>")]
pub fn ship_order(order: Json<Order>) -> Result<(), String> {
    let oid = validate_id(order.id.clone(), "oid")?;

    purchaseOrders::ship_po(oid)?;
    Ok(())
}

#[get("/status", format = "json", data = "<order>")]
pub fn get_status(order: Json<Order>) -> Result<RawHtml<String>, String> {
    let oid = validate_id(order.id.clone(), "oid")?;
    let cid = validate_id(order.id.clone(), "cid")?;
    let bid = validate_id(order.id.clone(), "bid")?;

    let addr = customers::get_customer_address(cid)?;

    let response_html = format!(
        "
        <html>
        <title>Order Status</title>
        </head>
        <body>
        <h1>Order Status</h1>
        <p>Order ID: {}</p>
        <p>Book ID: {}</p>
        <p>Customer ID: {}</p>
        <p>Shipping Address: {}</p>
        </body>
        </html>
    ",
        oid,
        bid,
        cid,
        &addr.as_str()
    );

    Ok(RawHtml(response_html.clone()))
}

fn validate_id(id: Option<i64>, label: &str) -> Result<i64, String> {
    //! makes sure a valid value is provided for cid/bid/oid
    let id = match id {
        Some(s) => s,
        None => return Err(format!("no {} provided",label))
    };
    
    if id <= 0 {
        Err(format!("{} must be a value greater than 0",label))
    } else {
        Ok(id)
    }
}
