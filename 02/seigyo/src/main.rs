fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4 or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // 型は全て同じでないとだめ
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    };
    println!("LIFTOFF!");

    let mut x = 0;
    loop {
        x += 1;
        if x >= 10 {
            break;
        }
    }
    dbg!(x);

    let mut x = 0;
    while {
        x += 1;
        x < 10
    } {}
    dbg!(x);

    let mut x = 0;
    'outer: loop {
        'inner: while {
            x += 1;
            if x > 5 {
                break 'outer;
            }
            x < 10
        }{}
    }
    dbg!(x);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let mut sum=0;
    for i in 1..100 {
        sum += i;
    }
    println!("sum = {}",sum);

    let s = String::from("hello");
    println!("{}", &s[0..2]);
    println!("{}", &s[0..=2]);
    println!("{}", &s[..2]);
    println!("{}", &s[3..s.len()]);
    println!("{}", &s[3..]);
    println!("{}", &s[..]);

    let a = [1,2,3,4,5];
    let a_slice = &a[1..3];
    println!("{:?}", a_slice);
}
