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
    let b_mut_ref2 = &mut b;　// b_mut_ref1のライフタイム開始
    *b_mut_ref2 = 20;
    println!("{}", b_mut_ref2); // b_mut_ref2のライフタイム終了
    println!("{}", b);
    // println!("{}", b_ref1); // エラー
    // println!("{}", b_mut_ref1); // エラー
    
}
