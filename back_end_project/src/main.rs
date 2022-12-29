use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

//function randomly picks a prod_name, price, quantity_left, and prod_Manufact and returns it
//as a tuple.
fn gen_prod()-> (&'static str, &'static str, &'static str, &'static str){
    //test data values
    let prod_names = ["Super Mario Galaxy", "MSI gaming monitor", "Xbox Series X", "PS5", "4090 RTX", 
        "Elden Ring", "Mario sticker", "Viking Hat", "Silk bean bag chair"];
    let prices = ["29.99", "15.99", "1234.99", "72.99", "99.99", "499.99", "899.99", "9.99", "0.99"];
    let quantities_left = ["10", "5", "1", "0", "100", "7", "13", "50", "25"];
    let prod_manufactors = ["Nintendo", "MSI","Microsoft", "Sony", "Nvidia", "FromSoftware",
         "Urban Outfitters", "Crate and Barrel", "Bestbuy"];

    //create the rng variable to generate our random numbers
    let mut rng = rand::thread_rng();

    //randomly create test data and return it
    let pn_choice = prod_names[rng.gen_range(0..9)];
    let p_choice = prices[rng.gen_range(0..9)];
    let ql_choice = quantities_left[rng.gen_range(0..9)];
    let pm_choice = prod_manufactors[rng.gen_range(0..9)];

    //return the newly generated product tuple
    return (pn_choice, p_choice, ql_choice, pm_choice);
}

//function randomly generates an order from the existing products and customers then 
//returns it as a tuple of 3 strs
fn gen_order()-> (String, String, String){
    //orders columns:
    // are  proid, custid, order_date, order_total
    //products:
    //1: xbox series x 72.99 
    //2: ps5 72.99
    //3: viking hat 899.99 
    //4: xbox series x 499.99
    //5: MSI gaming monitor 9.99 
    //6: Mario sticker 9.99 
    //7: MSI gaming monitor 0.99 
    //8: Xbox series x 899.99
    //9: Mario sticker 72.99 
    //10: Mario sticker 1234.99 
    //custid's range from 1-24
    //create the rng variable to generate our random numbers
    let mut rng = rand::thread_rng();

    let prices = [0.00,72.99, 72.99, 899.99, 499.99, 9.99, 9.99, 0.99, 899.99, 72.99, 1234.99];
    let product_ids = [1,2,3,4,5,6,7,8,9,10];
    let products_id = rng.gen_range(1..10);
    let cust_id = rng.gen_range(1..24);
    let order_total = prices[products_id];

    return (product_ids[products_id].to_string(), cust_id.to_string(), order_total.to_string());
 }

 //generates a random date
 //note: the days are only between 1-30
 fn gen_date() -> String{
    let mut rng = rand::thread_rng();
    let slash = "/";
    let month = rng.gen_range(1..12);
    let day = rng.gen_range(1..30);
    let year = rng.gen_range(2019..2023);
    let order_date = format!("{}{}{}{}{}", month, slash, day, slash, year).to_string();

    return order_date;
 }
fn main(){
    println!("-----Generating SQL insert statements for product table-----");
    //loop and generate the new random products and print out the sql for them
    let mut file = File::create("C:\\Users\\Admin\\Documents\\LearningRust\\back_end_project\\SQL\\OrderTable_TestData_Insert_Query.sql").expect("Unable to create file.");
    file.write(b"--SQL pre generated using ProductGenerator written in rust by Taittinger Gabelhart\n").expect("failed to write created by line.");
    for x in 0..100{
        //gen_Prod returns a tuple(id, prod_name, price, quantity_left, prod_manfact)
        let new_prod = gen_order();
        let str_val = "  VALUES(";
        let id: String = (x+1).to_string();
        let comma    = " , ";

        let line1 = "INSERT INTO public.orders(orderid, prodid, custid, order_date, order_total)\n";
        let line2 = str_val.to_owned() + &id + comma + &new_prod.0 + comma + &new_prod.1 + comma + &gen_date() + comma +  &new_prod.2 + ");\n";

        //now we write the query to the file
        file.write_all(line1.as_bytes()).expect("failed to write line 1.");
        file.write_all(line2.as_bytes()).expect("failed to write line 2.");
    }
    println!("-----End of Generation-----");
}