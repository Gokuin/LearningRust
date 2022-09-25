use std::io;

fn main() {
    //run through the chapter three examples form the book
    ex_One();
    ex_Two();
    ex_Three();
    ex_Four();
    ex_Five();
    ex_Six();
    ex_Seven();
    ex_Eight(8);
    ex_Nine(6,9);
    ex_Ten();
}

fn ex_One(){
//example 1 mutable variables
println!("Example One:");
let mut x = 5;
println!("The value of x is: {}", x);
x = 6;
println!("The value of x is: {}\n", x);
}

fn ex_Two(){
//example 2 using shadowing
println!("Example Two:");
let y = 5;
let y = y +1;
let y = y*2;
println!("The value of y is: {}\n", y);
}

fn ex_Three(){
    //example 3 floating point types
    println!("Example Three:");
    let z = 2.0; //f64 double precison float
    let w: f32 = 3.0; //single precison float
    println!("The value of z is: {}", z);
    println!("The value of w is: {}\n", w);
}

fn ex_Four(){
    //example 4 numeric operations
    println!("Example Four:");
    let sum = 5+10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 %5;
    println!("Sum: {}\nDifference: {}\nProduct: {}\nQuotient: {}\nRemainder: {}", sum, difference, product, quotient, remainder);
}

fn ex_Five(){
//example 5 tuple types
println!("\nExample Five:");
let tup:(i32, f64, u8) = (500, 6.4, 1);
//this is called deconstructing the tuple
let (a,b,c) = tup;
println!("The value of a: {}\nb: {}\nc: {}\n", a, b, c);
}

fn ex_Six(){
//example 6 accessing direct elements of a tuple
println!("Example Six:");
let x:(i32, f64, u8) = (500, 6.4, 1);
//accessing the tuple elements directly
let five = x.0;
let six = x.1;
let one = x.2;
println!("The value of a: {}\nb: {}\nc: {}\n", five, six, one);
}

fn ex_Seven(){
//example 7 arrays in rust
println!("Example Seven:");
let a = [1, 2, 3, 4, 5];
let b: [i32;5] = [6, 7, 8, 9, 10];
//let c = [3;5];///this makes a new array with 5 elemts all '3'

let first = a[0];
println!("Enter in an index in the array to get an number: ");
let mut index = String::new();
io::stdin().read_line(&mut index).expect("Failed to read a line:");
let index: usize = index.trim().parse().expect("Not a number");

//if the value of index is greater than a panic will occur this is a rust safety feature
let element = a[index];
println!("Value of [{}] is: {}", index, element);
}

fn ex_Eight(x:i32){
    println!("Example Eight:");
    println!("\nThe value of x is: {}",x);
}

fn ex_Nine(x:i32, y:i32){
    println!("Example Nine:");
    println!("The value of x is: {}, y: {}", x, y);
}

fn ex_Ten(){
println!("Example Ten:");
let dpg = 90;

let ki = {
    let dpg = 78;
    dpg+1
};
println!("Value of ki: {}", ki);
}