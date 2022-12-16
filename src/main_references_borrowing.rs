fn main() {
    mutable_references();
}

fn mutable_references(){
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem

    r3.push_str("string");
    println!("{}", r3);


}

/*
We call the action of creating a reference borrowing
 */
fn references_borrowing(){
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}