fn main(){
    println!("-----Generating SQL insert statements for product table-----");
    //loop and generate the new random products and print out the sql for them
    for x in 0..10{
        //gen_Prod returns a tuple(id, prod_name, price, quantity_left, prod_manfact)
        let mut new_Prod = gen_Prod();
        println!("INSERT INTO public.products(/n
            prodid, prod_name, price, quantity_left, prod_manufactor)");
        println!("  VALUES({}, {}, {}, {}, {});", (x + 1), new_Prod.0, new_Prod.1, new_Prod.2, new_Prod.3);
    }
    println!("-----End of Generation-----");
}

//function randomly picks a prod_name, price, quantity_left, and prod_Manufact and returns it
//as a tuple.
fn gen_Prod(){

}