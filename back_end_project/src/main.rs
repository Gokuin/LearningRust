use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn main(){
    println!("-----Generating SQL insert statements for product table-----");
    //loop and generate the new random products and print out the sql for them
    let mut file = File::create("C:\\Users\\Admin\\Documents\\LearningRust\\back_end_project\\SQL\\ProductTable_TestData_Insert_Query.sql").expect("Unable to create file.");
    file.write(b"--SQL pre generated using ProductGenerator written in rust by Taittinger Gabelhart\n").expect("failed to write created by line.");
    for x in 0..10{
        //gen_Prod returns a tuple(id, prod_name, price, quantity_left, prod_manfact)
        let new_prod = gen_prod();
        let str_val = "  VALUES(";
        let id: String = (x+1).to_string();
        let comma    = " , ";

        let line1 = "INSERT INTO public.products(prodid, prod_name, price, quantity_left, prod_manufactor)\n";
        let line2 = str_val.to_owned() + &id + comma + new_prod.0 + comma + new_prod.1 + comma + new_prod.2 + comma + new_prod.3 + ");\n";

        //now we write the query to the file
        file.write_all(line1.as_bytes()).expect("failed to write line 1.");
        file.write_all(line2.as_bytes()).expect("failed to write line 2.");
    }
    println!("-----End of Generation-----");
}

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