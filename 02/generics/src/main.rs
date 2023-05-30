fn add< T: std::ops::Add<Output=T> > (a: T, b: T) -> T {
    a+b
}

struct Point<T> {x: T, y:T}

enum Result<T,E>{
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn xy(self) -> (T,T){
        (self.x, self.y)
    }
}

impl Point<f64> {
    fn distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    println!("Hello, world!");

    let point = Point::<f64>{x:3., y:5.};

    let some_number = Some(5);
    let some_string = Some("a string");
    // let absent_number: Option<i32> = None;
}
