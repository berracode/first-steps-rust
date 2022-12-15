fn main() {

    //shadowing: you can declare a new variable with the same name as a previous variable.

    let x = 5;

    let x = x + 1; //the compiler will see this x variable because is the shadow

    { //inner scope starts
        let x = x * 2; //now, in inner scope, this x variable is what the compiler will see, but just in this inner scope
        println!("The value of x in the inner scope is: {x}");
    } //inner scope ends

    println!("The value of x is: {x}");
}

fn main1() {
    let mut x = 5; // in rust by default variables are immutable. With mut after let, our variable is mutable now.
    println!("The value of x is: {x}");
    x = 6; // x = 6 will throw an error.
    println!("The value of x is: {x}");
}