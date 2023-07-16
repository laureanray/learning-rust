use std::{fs::File, io::ErrorKind};

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
