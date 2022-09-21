use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //generate the number on start
    let hidden_num = rand::thread_rng().gen_range(1,101);
    loop {
        //introduce the game and then prompt the user for input
        println!("Try to guess the randomly generated number!");
        println!("Enter in a guess: ");

        //create a new mutable variable, by default "let guess" would be a immutable vairable 
        let mut guess = String::new();

        //now we grab input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //this uses rust shadowing to allow us to use the same variable name twice
        let guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        //now show the user back the taken in input as output
        println!("Your guess was: {}", guess);

        //check for a match
        match guess.cmp(&hidden_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            } 
    }
    }
    
}
