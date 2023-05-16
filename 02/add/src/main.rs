fn add(a: i32, b:i32) -> i32{
    a+b
}

fn main() {
    println!("{}",add(32,23));
    println!("{1} {} {0} {}",2,3);

    let people = "Rustaceans";
    println!("Hello {people}!");
    println!("{a} {c} {b}", a="a", b="b",c="c");

    println!("Hello {:5}!","x");
    println!("Hello {:1$}!","x",5);
    println!("Hello {1:0$}!",5,"x");

    println!("{:?}","12345");
    println!("{:?}","12.345");
    println!("{:?}","12345");

    let a = 2;
    let _b = dbg!(a * 2) + 1;

    let _c:i32 = 2;

    let d = 10;
    let d = d + 20;
    println!("{d}"); 

    let a = 10;
    println!("{a}");
    let a = "testtest";
    println!("{a}");

    let a = 10;

    { //local scope
        let mut a = 20;
        a += 30;
        println!("{}",a); // 50
    }
    println!("{}",a);
}
