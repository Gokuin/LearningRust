fn main(){
    println!("-----Generating SQL insert statements for product table-----");
    //loop and generate the new random products and print out the sql for them
    for x in 0..10{
        //gen_Prod returns a tuple(id, prod_name, price, quantity_left, prod_manfact)
        let mut new_Prod = gen_Prod();
    }
    println!("-----End of Generation-----");
}

fn gen_Prod(){

}