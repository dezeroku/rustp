fn main() {
    let a = 12;
    let b = 3 + a;
    for i in 0..10 {
        let x: i32 = 3;
    }

    //%assert a == 12
    let l = c(12);

    if l {
        let g: i32 = 3;
    }
    //%debug
    println!("{}", b);

    for i in 0..12 {}
    //%assert c(c(c(c(3)))) == 132
}

//%postcondition return_value
fn c(i: i32) -> bool {
    let x = true;
    if 1 == 2 {
        let y: bool = true;
    }
    x
}
