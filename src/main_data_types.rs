use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn main1(){

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("hello: {guess}");

    let x = 2.0; // f64

    let y: f32 = -3.0; // f32

    println!("x = {x} and y = {y}");

    // addition
    let sum: i64 = 5000000000 + 1000000000;
    println!("sum = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient = {quotient}");

    let truncated = -5 / 3; // Results in -1
    println!("truncated = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder = {remainder}");

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t = {t}, f = {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c = {c}, z = {z} {heart_eyed_cat}");

    //tuple

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    //arrays

    let a = [1, 2, 3, 4, 5];

    let b: [i32;5] = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("month of the year: {}", months[0]);

    let middle = &b[1..4]; // item in index: 1, 2, y 3
    let complete = &b[..]; //all items
    for i in middle {
        println!("complete: {}", i);

    }


    for i in &a {
        println!("Una referencia a {}", i);
    }

 

    let a2 = [3; 5]; // a contains five items that will all be set to the value 3


}