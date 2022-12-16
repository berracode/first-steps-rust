fn main(){
    passing_references_with_string();
}

fn passing_references_with_string(){
    let name = String::from("Juan Guillermo");
    println!("Name1 {}",name);
    //push_string(name);
    //println!("Name2 {}",name); this line isn't work because name is not owner now
    //solution:  push_string(&name);
    print_string(&name); //in this case print_string only can read name variable because that variable is not mutable

    let mut futbol_club = String::from("Liverpool");
    print_string(&futbol_club);
    push_string(&mut futbol_club);


}

fn push_string(futbol_param: &mut String){ //passing mutable reference = now futbol_club is owner but his value can be modified
    futbol_param.push_str(" Futbol Club LFC");
    println!("futbol: {}",futbol_param);

}

fn print_string(name_param: &String){
    println!("string: {}",name_param);

}

fn passing_references_with_numbers(){
    let mut x: u8 = 23;
    let mut y: u8 = 27;
    let x1: u8 = 23;
    let y1: u8 = 27;

    swap_numbers(x1, y1);
    //swap_numbers_by_reference(&mut x, &mut y);
    println!("MAIN. x = {x}, y = {y}");
}


fn swap_numbers_by_reference(x:  &mut u8,  y: &mut u8){
    let aux = *x;
    *x = *y;
    *y = aux;

    println!("3. x = {x}, y = {y}");

}

fn swap_numbers(mut x: u8, mut y: u8){
    let mut aux: u8 = 0;
    aux = x;
    x = y;
    y = aux;

    println!("1. x = {x}, y = {y}");

}