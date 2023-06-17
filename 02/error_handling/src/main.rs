use std::{fs::File, io::ErrorKind, error};

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Hoge {
    value: i32,
}

fn hoge() -> Option<i32> {
    let a = Some(10);
    // let a = None;
    let b = a?;
    Some(b)
}

// pub fn expect(self, msg: &str) -> T

fn main() {
    // panic!("crash");

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file: {:?}", e),
            }
        }
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let a = Some(10);
    let b = a.unwrap();
    println!("{}", b);

    let a = Some(Hoge{ value: 10 });
    let b = a.unwrap();
    // let c = a.unwrap();
    println!("{}",b.value);


}
