fn main() {
    let mut x = 5; // in rust by default variables are immutable. With mut after let, our variable is mutable now.
    println!("The value of x is: {x}");
    x = 6; // x = 6 will throw an error.
    println!("The value of x is: {x}");
}