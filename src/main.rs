use log::info;
use logging::log_init;

#[macro_use]
extern crate rocket;
extern crate serde;

mod db;
mod handlers;
mod logging;

#[launch]
fn rocket() -> _ {
    log_init();
    info!(target: "info", "server started");
    rocket::build()
        .mount("/books", routes![handlers::books::create_book])
        .mount("/books", routes![handlers::books::get_price])
        .mount("/customers", routes![handlers::customers::create_customer])
        .mount("/customers", routes![handlers::customers::get_balance])
        .mount("/customers", routes![handlers::customers::update_address])
        .mount("/orders", routes![handlers::orders::create_order])
        .mount("/orders", routes![handlers::orders::get_shipped])
        .mount("/orders", routes![handlers::orders::ship_order])
        .mount("/orders", routes![handlers::orders::get_status])
}
