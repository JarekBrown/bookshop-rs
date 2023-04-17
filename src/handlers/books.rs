use crate::db::books;
use log::warn;
use regex::Regex;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use titlecase::titlecase; // standardizes inputs to titlecase

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    id: Option<i64>,
    title: Option<String>,
    author: Option<String>,
    price: Option<f64>,
}
impl Book {}

#[post("/new", data = "<book>")]
pub fn create_book(book: Json<Book>) -> Result<(), String> {
    let title = validate_title(book.title.clone())?;
    let author = validate_auth(book.author.clone())?;
    let price = validate_price(book.price)?;

    books::create_book(title, author, price)?;
    Ok(())
}

// yes this throws a warning, it's how we're going it
// get methods can consume data in my world
// because putting and posting to get the price makes less
// sense in my mind
#[get("/price", format = "json", data = "<book>")]
pub fn get_price(book: Json<Book>) -> Result<Json<Book>, String> {
    let title = validate_title(book.title.clone())?;
    let author = validate_auth(book.author.clone())?;

    let bid = books::get_book_id(title.clone(), author.clone())?;
    let price = books::get_book_price(bid)?;
    Ok(Json(Book {
        id: Some(bid),
        title: Some(title),
        author: Some(author),
        price: Some(price),
    }))
}

fn validate_title(title: Option<String>) -> Result<String, String> {
    //! validation function for title field (unwraps Option<String>)
    let title = match title {
        Some(s) => s,
        None => {
            warn!(target: "warn", "title name validation failed: no title provided");
            return Err("no title provided".to_string());
        }
    };
    let re = Regex::new(r"\s+").expect("regex creation failed");
    let title = re.replace_all(&title, " ").to_string();
    if !title
        .chars()
        .all(|c| c.is_alphanumeric() || c.to_string() == " ")
    {
        warn!(target: "warn", "provided title is not alphanumeric");
        Err("title should be alpha-numeric".to_string())
    } else {
        Ok(titlecase(&title.to_lowercase()))
    }
}

fn validate_auth(author: Option<String>) -> Result<String, String> {
    //! validation function for author field (unwraps Option<String>)
    let auth = match author {
        Some(s) => s,
        None => {
            warn!(target: "warn", "author name validation failed: no author provided");
            return Err("no author provided".to_string());
        }
    };
    let re = Regex::new(r"\s+").expect("regex creation failed");
    let auth = re.replace_all(&auth, " ").to_string();
    if !auth
        .chars()
        .all(|c| c.is_alphanumeric() || c.to_string() == " ")
    {
        warn!(target: "warn", "provided author is not alphanumeric");
        Err("author should be alpha-numeric".to_string())
    } else {
        Ok(titlecase(&auth.to_lowercase()))
    }
}

fn validate_price(price: Option<f64>) -> Result<f64, String> {
    let price = match price {
        Some(s) => s,
        None => return Err("no price provided".to_string()),
    };
    if price >= 0.01 {
        Ok(price)
    } else {
        Err("price must be greater than 0".to_string())
    }
}
