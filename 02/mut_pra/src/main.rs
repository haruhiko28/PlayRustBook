fn main() {
    let mut a = 20;
    // a = 30;
    println!("{}",a);
    a = 30;
    println!("{}",a);

    let s = String::from("Hello World!");
    print!("{}",s);
    {
    let a = 30; 
    // 原本かつ不変
    let b = a;
    // b = 100; // cannot assign twice to immutable variable
    println!("{}",b);

    // // 原本かつ可変
    let c = 30;
    let mut d = c;
    d = 100;
    println!("{}",d);

    // // 仮かつ不変
    let e = 20;
    let f = &e;
    // f = 100;
    println!("{}",f);
    
    // // 仮かつ可変→可変参照
    let mut g = 30;
    // g = 40;
    let h_ref = &mut g;
    // g = 40;
    *h_ref = 40;
    println!("{}",h_ref);

    }
}
