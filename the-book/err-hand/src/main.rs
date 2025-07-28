use std::{fs::File, io::ErrorKind};

fn main() {
    let fres = File::open("hello.txt");

    let _ = match fres {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file {e:?}"),
            },
            _ => panic!("Problem opening file {error:?}"),
        },
    };
}
