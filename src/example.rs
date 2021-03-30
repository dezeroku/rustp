fn main() {
    let a: i32 = 12;
    let b: i32 = 3 + a;

    // %assert a == 12

    //%debug
    println!("{}", b);

    for i in 0..12 {}
    // %assert b == 15
    //%assert 15 > 13 && 7 == 7
}
