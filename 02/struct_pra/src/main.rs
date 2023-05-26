use std::fmt;

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

struct Password(String);

struct Hoge;

struct Rectangle {
    width: u32,
    height: u32,
}

struct Point1 {
    x: f64,
    y: f64,
}

enum IpAddKind {
    V4,
    V6,
}

impl fmt::Display for Password {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
    }
}

impl fmt::Display for Hoge {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hoge is unit-like struct")
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn _can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Point1 {
    fn new(x: f64, y: f64) -> Self{
        Self {x, y}
    }
}


fn main() {
    let black = Color(0,0,0);
    let Point(x,y,z) = Point(5,4,1);
    println!("{}, {}, {}",black.0, black.1, black.2);
    println!("{}, {}, {}",x, y, z);

    let a = String::from("123456789");
    println!("{}",a);

    let a = Password(String::from("123456789"));
    println!("{}",a);

    let hoge = Hoge{};
    println!("{}", hoge);

    let rect1 = Rectangle {width: 30, height: 20};
    println!("This area of the rectangle is {} square pixels", rect1.area());
    println!("This area of the rectangle is {} square pixels", Rectangle::area(&rect1));

    let a = Point1::new(3., 5.);
    println!("x={}, y={}", a.x, a.y);

    let four = IpAddKind::V4;
    let six  = IpAddKind::V6;
}
