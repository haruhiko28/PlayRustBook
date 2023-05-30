enum Coin {
    Penny,
    Nickel,
    Dimme,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dimme => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let five = Some(5);
    let six = Some(6);
    let none = plus_one(None);
    
    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let a: Option<String> = Some(String::from("hello"));
    let a = match a {
        // Some(ref x) => println!("{}",x),
        Some(x) => {println!("{}",x); Some(x)},
        None => None,
    };
    println!("{:?}",a);

    let num = Some(4); 
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}",x),
        None => (),
    }
    println!("{:?}", num);
}
