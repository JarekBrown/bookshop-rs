use super::db::connect;
use log::{error, info, warn};

pub fn create_customer(name: String, address: String) -> Result<i64, String> {
    let db = connect();
    let exist = exists(name.clone()).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if !exist {
        db.execute(
            "INSERT INTO customers (name, shippingAddress, accountBalance) VALUES (?1, ?2, 0.0)",
            &[&name, &address],
        )
        .expect("expected to be able to insert into Customers table");
        info!(target: "info", "new customer added: {}", name.clone());
        get_customer_id(name, address)
    } else {
        warn!(target: "warn", "customer already in database: {}", name);
        Err("customer already in database".to_string())
    }
}

pub fn get_customer_id(name: String, address: String) -> Result<i64, String> {
    let db = connect();
    let exist = exists(name.clone()).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        let mut stmt = db
            .prepare("SELECT id FROM customers WHERE name = ?1 AND shippingAddress = ?2")
            .expect("expected to be able to select from Customers table");
        let mut rows = stmt
            .query_map(&[&name, &address], |row| row.get(0))
            .expect("expected to be able to get id from Customers table");
        let id = rows
            .next()
            .expect("expected a value in the row")
            .expect("problem getting cid from database");
        Ok(id)
    } else {
        warn!(target: "warn", "failed to get cid: {}", name);
        Err("customer does not exist in database".to_string())
    }
}

pub fn get_customer_address(cid: i64) -> Result<String, String> {
    let db = connect();
    let exist = exists_id(cid).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        let mut stmt = db
            .prepare("SELECT shippingAddress FROM customers WHERE id = ?1")
            .expect("expected to be able to select from Customers table");
        let mut rows = stmt
            .query_map(&[&cid], |row| row.get(0))
            .expect("expected to be able to get shippingAddress from Customers table");
        let address = rows
            .next()
            .expect("expected a value in the row")
            .expect("problem getting address from database");
        Ok(address)
    } else {
        warn!(target: "warn", "failed to get customer address: {}", cid);
        Err("cid does not exist in database".to_string())
    }
}

pub fn update_customer_address(cid: i64, address: String) -> Result<(), String> {
    let db = connect();
    let exist = exists_id(cid).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        db.execute(
            "UPDATE customers SET shippingAddress = ?1 WHERE id = ?2",
            &[&address, &cid.to_string()],
        )
        .expect("expected to be able to update Customers table");
        Ok(())
    } else {
        warn!(target: "warn", "failed to get customer address: {}", cid);
        Err("cid does not exist in database".to_string())
    }
}

pub fn customer_balance(cid: i64) -> Result<f64, String> {
    let db = connect();
    let exist = exists_id(cid).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        let mut stmt = db
            .prepare("SELECT accountBalance FROM customers WHERE id = ?1")
            .expect("expected to be able to select from Customers table");
        let mut rows = stmt
            .query_map(&[&cid], |row| row.get(0))
            .expect("expected to be able to get accountBalance from Customers table");
        let balance = rows
            .next()
            .expect("expected a value in the row")
            .expect("problem getting balance from database");
        Ok(balance)
    } else {
        warn!(target: "warn", "failed to get customer balance: {}", cid);
        Err("cid does not exist in database".to_string())
    }
}

fn exists(name: String) -> Result<bool, rusqlite::Error> {
    //! checks that customer exists
    let conn = connect();
    let check = conn
        .prepare("SELECT id FROM customers WHERE name = ?1")
        .expect("expected to be able to select from Books table")
        .exists(&[&name])?;
    Ok(check)
}

fn exists_id(cid: i64) -> Result<bool, rusqlite::Error> {
    //! same functionality as exists() but when only cid is provided
    let conn = connect();
    let check = conn
        .prepare("SELECT name FROM customers WHERE id = ?1")
        .expect("expected to be able to select from Books table")
        .exists(&[&cid])?;
    Ok(check)
}
