fn main(){
    let mut x: u8 = 23;
    let mut y: u8 = 27;

    //swap_numbers(x, y);
    swap_numbers_by_reference(&mut x, &mut y);
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