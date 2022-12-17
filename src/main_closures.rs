#![allow(unreachable_code, unused_variables, dead_code)]


//closures: funcion que es definida en linea  (inline)
fn main(){

    let sum = |x|{x+1};

    println!("sum: {}", sum(4));
    basic_option();

}

fn basic_option(){
    let name: Option<String> = Some("sss".to_string());

    match name {
        None => println!("Name is null"),
        Some(va) => println!("{} ", va)
    };
}

fn plus_one(num: i32) -> i32{
    num+1

}