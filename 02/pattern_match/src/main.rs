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

    let (x, y, z) = (1,2,3);
    let [a, b, c] = [4,5,6];
    let (i, _ , _) = (7,8,9);
    println!("xyz={} {} {}", x, y, z);
    println!("abc={} {} {}", a, b, c);
    println!(" i ={}",i);
    
    let point = (3,5);
    print_coordinates(&point);

    let a = Account {
        name: String::from("name"), pass:String::from("pass")
    };

    let Account {ref name, ref pass} = a;
    println!("{} {}", name, pass);
    println!("{} {}", a.name, a.pass);

    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }

}

fn print_coordinates(&(x,y): &(i32,i32)){
    println!("location: {} {}", x, y);
}

struct Account {name: String, pass: String}