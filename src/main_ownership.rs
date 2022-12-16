fn main(){

    //arrays();
    //owner_with_heap();
    //copy_and_ownership();
    gives_and_takes_ownership();

}

fn gives_and_takes_ownership(){
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    println!("s1: {s1}");

    let s2 = String::from("hello");     // s2 comes into scope

    println!("s2: {s2}");

    let s3 = takes_and_gives_back(s2);  // s2 is moved into

    //println!("s2: {s2}"); failed
    println!("s3: {s3}");


}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn copy_and_ownership(){
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    //println!("{s}"); failed

    let x = 5;                      // x comes into scope

    makes_copy(x);
    println!("other: {x}");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // 

fn owner_with_heap(){
    let name = String::from("Cocacola");
    println!("{}",name);
    let name2 = name;
    //println!("{} {}",name, name2); failed


}

fn owner_str(){
    let name = "my aaaname";
    println!("{}", name);
    let name2 = name;
    println!("{} {}",name, name2);
}

fn arrays_with_string(){

}



/*
fixed size, let number: [&str; 3] =>[dataype; size]
*/
fn arrays(){

    let  array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let three_to_five = &array_of_ten[2..5];

    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];


    println!("Tres a cinco: {:?}, comienza en el segundo: {:?}, finaliza en el quinto: {:?}, todo: {:?}", three_to_five, start_at_two, end_at_five, everything);



    let mut numbers = ["one", "two", "three"];

    numbers[0] = "zero";

    for i in numbers {
        println!("item: {}",i);
    }

    numbers[1] = "oneplus";

    for i in numbers {
        println!("item: {}",i);
    }


}