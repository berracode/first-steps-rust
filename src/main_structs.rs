struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[derive(Debug)]
struct User{
    name: String,
    email: String,
    year: u16,
    active: bool
}

impl User {

    fn new(name: String, email: String, year: u16, active: bool) -> Self{
        Self{
            name,
            email,
            year,
            active
        }
    }
    
    fn get_age(&self) -> u16{
        let year = 2022;
        year - self.year
    }

    fn major_than(&self, other_user: &User) -> bool{
        self.get_age() > other_user.get_age()

    }
}

fn main(){
    working_with_structs();


    working_with_tuple_structs();

}

fn working_with_tuple_structs(){
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black {}",black.0);

}

fn working_with_structs(){
    let mut user = User{
        name: "Juan".to_string(),
        email: String::from("email@email"),
        year: 1998,
        active: true
    };
    println!("my user with display {:?}",user);
    user.active = false;
    print_user(&user);

    let mut user2 = create_user("Juan Pedro".to_string(), "Emial2@".to_string());

    let user3 = User{
        ..user2
    };

    print_user(&user3);

    println!("Age {}",user3.get_age());

    println!("{} is older than {}? {}", user.name, user3.name, user.major_than(&user3));

    let user4 = User::new(String::from("Kelly"), String::from("Kelly@gmail.com"), 1994, true);

    println!("{} is older than {}? {}", user4.name, user.name, user4.major_than(&user));



}

fn create_user(name: String, email: String) -> User{
    return User{
        name,
        email,
        year: 1981,
        active: false
    };
}

fn print_user(user: &User){
    println!("User {}, year {}, email {}, active {}",user.name, user.year, user.email, user.active);
}