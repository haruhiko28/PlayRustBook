fn main() {
    let a = 10;
    let b = a;
    println!("{} {}", a, b);

    let a = 10;
    let a_ref = &a;
    let a_ref_copy = a_ref;
    println!("{}, {}, {}", a, a_ref, a_ref_copy);

    let mut a = 10;
    let a_mut_ref = &mut a;
    let _a_mut_ref_move = a_mut_ref;
    // println!("{}", a_mut_ref); 

    let mut a = 10;
    let a_mut_ref = &mut a;
    let a_mut_ref_move = a_mut_ref;
    println!("{}", a_mut_ref_move);

    let _s = String::from("hello");
    // copy_trait_check(s);

    let a = 10;
    copy_trait_check(a);

    let a = 42;
    let ref_ref_ref_a = &&&a;
    let ref_a = **ref_ref_ref_a;
    let b = *ref_a;
    println!("{} {}", a, b);

    let a = 10;
    let _a_ref = &a;
    let _a_ref_ref = &a_ref;
    // println!("{}", a==a_ref);

    // println!("{}", a_ref_ref==a_ref);

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

fn copy_trait_check<T: Copy>(_: T){}
