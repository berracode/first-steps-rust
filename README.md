# Rust for Beginners

These are my first steps in rust. I'll all write that I consider important to remember or everything I consider important.


## Mutable reference
By default in rust the variables are immutable. 

In the example code below, there is a fuction that swaps two numbers.

```rust
fn main(){
    let mut x: u8 = 23;
    let mut y: u8 = 27;
    println!("main_before_swap. x = {x}, y = {y}");
    swap_numbers_by_reference(&mut x, &mut y);
    println!("main_after_swap. x = {x}, y = {y}");


}

fn swap_numbers_by_reference(x:  &mut u8,  y: &mut u8){
    let aux = *x;
    *x = *y;
    *y = aux;
}
```

Our `x` and `y` variable are mutable. Our function `swap_numbers_by_reference` receives this variables like mutable reference variables it means that inside our function we could change the value our variables and this change will be seen from our `main` function. 

On the other hand if our variables were passing like shared references we could only read these variables. 



## Data types

![alt text](./resources/integer_type_rust.png)

![alt text](./resources/integer_literal.png)


