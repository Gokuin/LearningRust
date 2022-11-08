use std::fs::File;
use std::io::prelude::*;

fn main(){
    ///NOTE: should change this so insead of printing it writes to a file instead
    println!("-----Generating SQL insert statements for product table-----");
    //loop and generate the new random products and print out the sql for them
    let mut line1 = "";
    let mut line2 = "";
    let mut file = File::create("C:\\Users\\Admin\\Documents\\LearningRust\\back_end_project\\SQL\\ProductTable_TestData_Insert_Query.sql")?;
    for x in 0..10{
        //gen_Prod returns a tuple(id, prod_name, price, quantity_left, prod_manfact)
        let mut new_Prod = gen_Prod();
        line1 = "INSERT INTO public.products(/n
            prodid, prod_name, price, quantity_left, prod_manufactor)";
        line2 = "  VALUES({}, {}, {}, {}, {});", (x + 1), new_Prod.0, new_Prod.1, new_Prod.2, new_Prod.3;
        file.write_all(line1)?;
        file.write_all(line2)?;
        OK(())
    }
    println!("-----End of Generation-----");
}

//function randomly picks a prod_name, price, quantity_left, and prod_Manufact and returns it
//as a tuple.
fn gen_Prod(){
    //test data values
    let prod_names = ["Super Mario Galaxy", "MSI gaming monitor", "Xbox Series X", "PS5", "4090 RTX", 
        "Elden Ring", "Mario sticker", "Viking Hat", "Silk bean bag chair"];
    let prices = [29.99, 15.99, 1234.99, 72.99, 99.99, 499.99, 899.99, 9.99, .99];
    let quantities_left = [10, 5, 1, 0, 100, 7, 13, 50, 25];
    let prod_manufactors = ["Nintendo", "MSI","Microsoft", "Sony", "Nvidia", "FromSoftware",
         "Urban Outfitters", "Crate and Barrel", "Bestbuy"];

    //randomly create test data and return it
    let mut pn_choice = rand::thread_rng().gen_range(0,10);
    let mut p_choice = rand::thread_rng().gen_range(0,10);
    let mut ql_choice = rand::thread_rng().gen_range(0,10);
    let mut pm_choice = rand::thread_rng().gen_range(0,10);
    
    //return the newly generated product tuple
    let mut prod:(prod_names[pn_choice], prices[p_choice], quantities_left[ql_choice], prod_manufactors[pm_choice]);
}