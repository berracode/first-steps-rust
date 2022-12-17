const  PI: f64 = 3.1415926535;

struct Rectangle{
    width: Option<f64>,
    higth: Option<f64>
}

struct Circle{
    diameter: f64
}


/*
A type’s behavior consists of the methods we can call on that type. Different types share the same behavior 
if we can call the same methods on all of those types. Trait definitions are a way to group method signatures 
together to define a set of behaviors necessary to accomplish some purpose.
*/
trait Figure {

    fn area(&self)->f64;
}

impl Rectangle {

    fn new() -> Self{
        Self{
            width: None,
            higth: None
        }
    }
}

impl Figure for Rectangle{

    fn area(&self)->f64 {
        if self.higth.is_some() && self.width.is_some() {
            self.higth.unwrap()*self.width.unwrap()
        }else{
            0.0
        }
    }
}

impl Figure for Circle {
    fn area(&self)->f64 {
        PI*(self.diameter/2.0)*(self.diameter/2.0)
    }
}

fn main(){
    let mut rect1 = Rectangle::new();
    rect1.higth = Some(23.0);
    rect1.width = Some(12.0);
    println!("area rec1: {}", rect1.area());


}