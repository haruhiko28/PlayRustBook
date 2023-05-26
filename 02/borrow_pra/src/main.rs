fn main() {
    let a = 10;
    let aref1 = &a;
    let aref2 = &a;
    println!("{} {} {}", a, aref1, aref2);

    let mut b = 10;
    let b_ref1 = &b;
    println!("{}", b_ref1);
    let b_mut_ref1 = &mut b;
    println!("{}", b_mut_ref1);
    let b_mut_ref2 = &mut b; // b_mut_ref1のライフタイム開始
    *b_mut_ref2 = 20;
    println!("{}", b_mut_ref2); // b_mut_ref2のライフタイム終了
    println!("{}", b);
    // println!("{}", b_ref1); // エラー
    // println!("{}", b_mut_ref1); // エラー
    
    let mut s = "Hello World!".to_string();
    let _first = &mut s[..5];
    let last = &mut s[5..];
    // first.make_ascii_uppercase(); // second mutable borrow occurs here
    last.make_ascii_lowercase();
    println!("{}",s);

    let mut s = "Hello World!".to_string();
    let (first, last) = s.split_at_mut(5);
    first.make_ascii_uppercase();
    last.make_ascii_lowercase();
    println!("{}",s);
}
