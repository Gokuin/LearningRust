// import our Date object from the routes/date module
use crate::routes::database::{Count, Res};
use postgres::{Client, Error, NoTls};

//function will hit the postgres database in the back end 
//then returns a Count struct of the current inventory items
pub fn get_current_inventory_count() -> Count {
    //future wise this is where I would connect to the database and return the count of all the products
    let count = 10;
    let current_count = Count {
        count
    };
    current_count
}

//function attempts to connect to the database that hosts our test data it then returns the result
pub fn attempt_connection() -> Res {
    //currently panicing when attempting connection
    let r = true;

    let client = Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)
        .expect("The connection to the database failed... it shouldnt");
    

    let connect_res = Res {
        r
    };
    
    connect_res
}