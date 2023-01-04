use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    // panic!("crash and burn");
    // example syntax using match as a means to handle unwrapping the Result<T, E> (getting the object inside)
    let greeting_file_result =  File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }, 
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
   
         }  
    };
    // unwrap returns the object or calls panic if the object contains Err(E)
    let greeting_file = File::open("hello.txt").unwrap();
    
    // expect syntax lets you specify the panic!() call message
    let mut username_file = File::open("user.txt").expect("hello.txt should be included in this project");

    let username = match read_username_from_file(&mut username_file) {
        Ok(username) => username,
        Err(_) => panic!("No username found"),
    };

    print!("Username: {username}");
}

fn read_username_from_file(file: &mut File) -> Result<String, io::Error> {

    let mut username = String::new();

    // ? operator: identical to  
    // match file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e)
    // }   
    file.read_to_string(&mut username)?;
    Ok(username)


}
