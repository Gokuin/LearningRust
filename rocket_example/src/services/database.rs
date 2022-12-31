// chrono is a time library for Rust
//use chrono::Datelike;

//use rocket::serde::json::{Json};
// import our Date object from the routes/date module
use crate::routes::database::Count;

pub fn get_current_inventory_count() -> Count {
    //future wise this is where I would connect to the database and return the count of all the products
    let count = 10;
    let current_count = Count {
        count
    };
    current_count
}