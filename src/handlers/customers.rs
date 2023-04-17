use log::warn;
use regex::Regex;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use titlecase::titlecase;

use crate::db::customers;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    id: Option<i64>,
    name: Option<String>,
    shipping_address: Option<String>,
    account_balance: Option<f64>,
}

#[post("/new", data = "<customer>")]
pub fn create_customer(customer: Json<Customer>) -> Result<(), String> {
    let name = validate_name(customer.name.clone())?;
    let address = validate_addr(customer.shipping_address.clone())?;

    customers::create_customer(name, address)?;
    Ok(())
}

#[post("/updateAddress", data = "<customer>")]
pub fn update_address(customer: Json<Customer>) -> Result<(), String> {
    let cid = validate_cid(customer.id)?;
    let address = validate_addr(customer.shipping_address.clone())?;

    customers::update_customer_address(cid, address)?;
    Ok(())
}

#[get("/balance", format = "json", data = "<customer>")]
pub fn get_balance(customer: Json<Customer>) -> Result<Json<Customer>, String> {
    let name = validate_name(customer.name.clone())?;
    let address = validate_addr(customer.shipping_address.clone())?;

    let cid = customers::get_customer_id(name, address)?;
    let balance = customers::customer_balance(cid)?;
    Ok(Json(Customer {
        id: None,
        name: None,
        shipping_address: None,
        account_balance: Some(balance),
    }))
}

fn validate_name(name: Option<String>) -> Result<String, String> {
    //! validation function for name field (unwraps Option<String>)
    let name = match name {
        Some(s) => s,
        None => {
            warn!(target: "warn", "customer name validation failed: no name provided");
            return Err("no name provided".to_string());
        }
    };
    let re = Regex::new(r"\s+").expect("regex creation failed");
    let name = re.replace_all(&name, " ").to_string();
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c.to_string() == " ")
    {
        warn!(target: "warn", "provided customer name is not alphanumeric");
        Err("name should be alpha-numeric".to_string())
    } else {
        Ok(titlecase(&name.to_lowercase()))
    }
}

fn validate_addr(addr: Option<String>) -> Result<String, String> {
    //! validation function for address field (unwraps Option<String>)
    let addr = match addr {
        Some(s) => s,
        None => {
            warn!(target: "warn", "customer address validation failed: no address provided");
            return Err("no address provided".to_string());
        }
    };
    let re = Regex::new(r"\s+").expect("regex creation failed");
    let name = re.replace_all(&addr, " ").to_string();
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c.to_string() == " ")
    {
        warn!(target: "warn", "provided customer address is not alphanumeric");
        Err("addr should be alpha-numeric".to_string())
    } else {
        Ok(titlecase(&addr.to_lowercase()))
    }
}

fn validate_cid(cid: Option<i64>) -> Result<i64, String> {
    //! makes sure a positive value is provided for cid
    let cid = match cid {
        Some(s) => s,
        None => return Err("No id provided".to_string()),
    };
    if cid <= 0 {
        Err("cid must be a value greater than 0".to_string())
    } else {
        Ok(cid)
    }
}
