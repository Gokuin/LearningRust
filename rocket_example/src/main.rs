// import Rocket
#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;

//use rocket_contrib::databases::diesel;

// add our routes and services modules
mod routes;
mod services;

// import our routes
use routes::date::get_current_date;
use routes::date::date_plus_month;
use routes::database::get_current_inventory_count;
use routes::database::attempt_connection;

// start the web server and mount our get route at "/api". Can replace /api with anything
// or just leave it as "/" as the default location
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_current_date, date_plus_month, get_current_inventory_count, attempt_connection])
}