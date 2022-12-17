#![allow(unreachable_code, unused_variables, dead_code)]


#[derive(Default, Debug)]
enum UserRole{
    #[default]
    BASIC,
    ADMIN
}

#[derive(Default, Debug)]

enum WebSite {
    #[default]
    None,
    URL(String),
    INSTAGRAM(String)
}
#[derive(Default, Debug)]
struct User{
    name: String,
    age: u8,
    user_role: UserRole,
    web_site: WebSite
}

fn main(){

    let user = User::default();

    println!("user: {:?}", user);

}