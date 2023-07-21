use std::{fs::File, io::{ErrorKind, self, Read}};

fn main() {
    let open_file_result = File::open("hello.txt");

    let greeting_file = match open_file_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(c) => c,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

    println!("{:?}", greeting_file.metadata().ok());
}


fn alternative() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        match error.kind() {
            ErrorKind::NotFound => File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            }),
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username  = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
