use super::db::connect;
use log::{error, info, warn};

pub fn create_purchase_order(cid: i64, bid: i64) -> Result<i64, String> {
    let db = connect();
    let exist = exists_id(cid, bid).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if !exist {
        db.execute(
            "INSERT INTO PurchaseOrders (customerId, bookId, shipped) VALUES (?1, ?2, 0)",
            &[&cid, &bid],
        )
        .expect("expected to be able to insert into PurchaseOrders table");
        info!(target: "info", "new order created (cid, bid): {}, {}", cid, bid);
        get_purchase_order_id(cid, bid)
    } else {
        warn!(target: "warn", "order already in database (cid, bid): {}, {}", cid, bid);
        Err("order already in database".to_string())
    }
}

pub fn get_purchase_order_id(cid: i64, bid: i64) -> Result<i64, String> {
    let db = connect();
    let exist = exists_id(cid, bid).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        let mut stmt = db
            .prepare("SELECT id FROM PurchaseOrders WHERE customerId = ?1 AND bookId = ?2")
            .expect("expected to be able to select from PurchaseOrders table");
        let mut rows = stmt
            .query_map(&[&cid, &bid], |row| row.get(0))
            .expect("expected to be able to get id from PurchaseOrders table");
        let id = rows
            .next()
            .expect("expected a value in the row")
            .expect("problem getting poid from database");
        Ok(id)
    } else {
        warn!(target: "warn", "failed to get poid (cid, bid): {}, {}", cid, bid);
        Err("purchase order does not exist in database".to_string())
    }
}

pub fn is_po_shipped(poid: i64) -> Result<i64, String> {
    let db = connect();
    let exist = exists_shipped(poid).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        let mut stmt = db
            .prepare("SELECT shipped FROM PurchaseOrders WHERE id = ?1")
            .expect("expected to be able to select from PurchaseOrders table");
        let mut rows = stmt
            .query_map(&[&poid], |row| row.get(0))
            .expect("expected to be able to get shipped from PurchaseOrders table");
        let shipped: i64 = rows
            .next()
            .expect("expected a value in the row")
            .expect("problem getting shipped from database");
        Ok(shipped)
    } else {
        warn!(target: "warn", "poid not in database: {}", poid);
        Err("purchase order does not exist in database".to_string())
    }
}

pub fn ship_po(poid: i64) -> Result<(), String> {
    let db = connect();
    let exist = exists_shipped(poid).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        db.execute(
            "UPDATE PurchaseOrders SET shipped = 1 WHERE id = ?1",
            &[&poid],
        )
        .expect("expected to be able to update PurchaseOrders table");
        Ok(())
    } else {
        warn!(target: "warn", "poid not in database: {}", poid);
        Err("purchase order does not exist in database".to_string())
    }
}

fn exists_id(cid: i64, bid: i64) -> Result<bool, rusqlite::Error> {
    //! checks that the cid and bid exist in database
    let conn = connect();
    let check = conn
        .prepare("SELECT id FROM PurchaseOrders WHERE customerId = ?1 AND bookId = ?2")
        .expect("expected to be able to select from Books table")
        .exists(&[&cid, &bid])?;
    Ok(check)
}

fn exists_shipped(poid: i64) -> Result<bool, rusqlite::Error> {
    //! same functionality as exists_id() but when only poid is provided
    let conn = connect();
    let check = conn
        .prepare("SELECT shipped FROM PurchaseOrders WHERE id = ?1")
        .expect("expected to be able to select from Books table")
        .exists(&[&poid])?;
    Ok(check)
}
