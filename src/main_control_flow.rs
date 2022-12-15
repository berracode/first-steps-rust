fn main(){
    repetition_with_loops();
    
    //if_expression();
}

/*
Rust has three kinds of loops: loop, while, and for. Let’s try each one.

The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
 */
fn repetition_with_loops(){
    //infinite_loop();
    //returning_values_from_loop();
    //loops_labels();
    //conditional_loops_with_while();
    //looping_through_a_collection_using_while();
    looping_through_a_collection_using_for();
    looping_through_a_collection_using_for_bit_nicer();
}

fn looping_through_a_collection_using_for_bit_nicer(){
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn looping_through_a_collection_using_for(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn looping_through_a_collection_using_while(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn conditional_loops_with_while(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loops_labels(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn returning_values_from_loop(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn infinite_loop(){
    loop {
        println!("again!");
    }
}

fn if_expression(){
    let number = 3;
    let number2 = 2;

    if number < 5 && number2 == 5{
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}