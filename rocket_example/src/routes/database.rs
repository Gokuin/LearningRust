use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

// import services module
use crate::services;

// create a struct to hold our Date data
// need serialize/deserialize to convert to/from JSON
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Count {
    pub count: u32
}

// create get-current-inventory-count route under /database and call get_current_date service which will return a Date object
// route returns a Date object converted to JSON
#[get("/database/get-current-inventory-count")]
pub fn get_current_inventory_count() -> Json<Count> {
    Json(services::database::get_current_inventory_count())
}
