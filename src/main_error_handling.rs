use std::{fs::File, io::{ErrorKind, BufReader, BufRead, self, Read}};

fn main() {
    //recoverable_errors_with_result();
    //matching_on_different_errors();
    //matching_on_different_errors_alternative();
    //propagating_errors();
    //propagating_errors_with_shortcut();
    propagating_errors_with_shortcut_with_chainning();
}

fn propagating_errors_with_shortcut_with_chainning(){
    let user = read_username_from_file_chainning();
    println!("error: {}", user.is_err());
}


fn read_username_from_file_chainning() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

/*
A Shortcut for Propagating Errors: the ? Operator
*/
fn propagating_errors_with_shortcut(){
    let user = read_username_from_file_shortcut();
    println!("error: {}", user.is_err());
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn propagating_errors(){
    let read_user = read_username_from_file();
    println!("something: {}",read_user.is_ok());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn matching_on_different_errors_alternative(){
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn matching_on_different_errors(){
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    

}

fn recoverable_errors_with_result(){

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("Pragam continue");

}
