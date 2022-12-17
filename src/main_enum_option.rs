
struct User{
    name: String,
    age: Option<i32>
}

impl User {

    fn get_age(&self) -> i32{
        self.age.unwrap_or_default()
    }
}

fn main(){

   let user = User{
    name: "Juan".to_string(),
    age: None
   };

   let age = user.get_age();
   /*match age {
    Some(age) => println!("age = {} ", age),
    _ => println!("Name is null"), //means all other cases
    };*/
    println!("age = {} ", age);
}


fn basic_option(){
    let name: Option<String> = "None";

    match name {
        None => println!("Name is null"),
        Some(va) => println!("{} ", va)
    };
}