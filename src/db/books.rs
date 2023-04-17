use super::db::connect;
use log::{error, info, warn};

pub fn create_book(title: String, author: String, price: f64) -> Result<(), String> {
    let db = connect();
    let exist = exists(title.clone(), author.clone()).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if !exist {
        db.execute(
            "INSERT INTO books (title, author, price) VALUES (?1, ?2, ?3)",
            [&title, &author, &format!("{}", price)],
        )
        .expect("expected to be able to insert into Books table");
        info!(target: "info", "book created: {} by {} for {}", title, author, price);
        Ok(())
    } else {
        warn!(target: "warn", "pre-existing book entered for creation (was not added): {} by {} for {}", title, author, price);
        Err("book already exists in database".to_string())
    }
}

pub fn get_book_id(title: String, author: String) -> Result<i64, String> {
    let db = connect();
    let exist = exists(title.clone(), author.clone()).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        let mut stmt = db
            .prepare("SELECT id FROM books WHERE title = ?1 AND author = ?2")
            .expect("expected to be able to select from Books table");
        let mut rows = stmt
            .query_map([&title, &author], |row| row.get(0))
            .expect("expected to be able to get id from Books table");
        let id = rows
            .next()
            .expect("expected a value in the row")
            .expect("problem getting bid from database");
        Ok(id)
    } else {
        warn!(target: "warn", "failed to get book id: {} by {}", title, author);
        Err("book does not exist in database".to_string())
    }
}

pub fn get_book_price(bid: i64) -> Result<f64, String> {
    let db = connect();
    let exist = exists_id(bid).unwrap_or_else(|e| {
        error!(target: "error", "statement exists check error: {}", e);
        panic!("connection with database failure")
    });
    if exist {
        let mut stmt = db
            .prepare("SELECT price FROM books WHERE id = ?1")
            .expect("expected to be able to select from Books table");
        let mut rows = stmt
            .query_map([&bid], |row| row.get(0))
            .expect("expected to be able to get price from Books table");
        let price = rows
            .next()
            .expect("expected a value in the row")
            .expect("problem getting price from database");
        Ok(price)
    } else {
        warn!(target: "warn", "failed to get book price: {}", bid);
        Err("bid does not exist in database".to_string())
    }
}

fn exists(title: String, author: String) -> Result<bool, rusqlite::Error> {
    //! checks if requested item exists in database
    let conn = connect();
    let check = conn
        .prepare("SELECT id FROM books WHERE title = ?1 AND author = ?2")
        .expect("expected to be able to select from Books table")
        .exists([&title, &author])?;
    Ok(check)
}

fn exists_id(bid: i64) -> Result<bool, rusqlite::Error> {
    //! same functionality as exists() but when only bid is provided
    let conn = connect();
    let check = conn
        .prepare("SELECT price FROM books WHERE id = ?1")
        .expect("expected to be able to select from Books table")
        .exists([&bid])?;
    Ok(check)
}
