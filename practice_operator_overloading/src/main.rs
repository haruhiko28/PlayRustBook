use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output =  Point;

    fn add (self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 :Point = Point{x:1,y:2};
    let p2 :Point = Point{x:1,y:2};
    let p3 :Point = p1.add(p2);

    // Copyトレイトがないとエラー→p1 の所有権を add メソッドに移動させるため
    println!("x:{}, y:{}", p1.x, p1.y);
    
    println!("x:{}, y:{}", p2.x, p2.y);
    println!("x:{}, y:{}", p3.x, p3.y);
}
