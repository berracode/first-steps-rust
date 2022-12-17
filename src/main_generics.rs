use std::fmt::Display;
use std::cmp::PartialOrd;
use std::ops::Mul;
use std::marker::Copy;


struct  Rectangle<T: Mul<Output = T>>{
    width: T,
    higth: T
}

impl<T> Rectangle<T>
where
    T: Copy + Mul<Output = T>
{
    fn area(&self) -> T {
        let h = self.higth;
        let w = self.width;
        h*w
    }
}

fn main(){
    let rectangle = Rectangle{
        width: 7,
        higth: 14
    };

    println!("area of rectangle is: {}", rectangle.area());

    compare_print("Listen!", 2, 3);

}

fn compare_print<T: Display, U: Display + PartialOrd>(text: T, num_a: U, num_b: U){
    println!("{} ... {} is major than {} ? {}", text, num_a, num_b, num_a > num_b);
}

fn compare_print_with_where<T, U>(text: T, num_a: U, num_b: U)
where
    T: Display, U: Display + PartialOrd
{
    println!("{} ... {} is major than {} ? {}", text, num_a, num_b, num_a > num_b);
}