fn main() {
    let a: i32 = 12;
    let b: i32 = 3 + a;

    // %assert a == 12

    //%debug
    println!("{}", b);

    for i in 0..12 {}
    //%assert a == 12
    //%assert b == 15
    //%assert b == a + 3
    // %assert 15 > 13 && 7 == 7
}

fn remainder(x: i32, y: i32) -> (i32, i32) {
    // TODO: move this to the precondition
    //%assume x >= 0
    //%assume y >= 0

    // First program, proved by Hoare
    //let x: i32 = 27;
    //let y: i32 = 4;

    let mut quo: i32 = 0;
    let mut rem: i32 = x;

    //%assert quo * y + rem == x
    while rem >= y {
        rem = rem - y;
        quo = quo + 1;
    }

    // TODO: This is not actually being proved yet, the while above just gets ignored
    //%assert quo * y + rem == x

    //%debug
    println!("{} {}: {} {}", x, y, quo, rem);

    // End of the first program
    (quo, rem)
}

//%precondition a==0 && xdd == 13
fn c(a: i32, xdd: i32) {
    // Assumes should be just taken into the context it seems. Just asserting them will be enough?

    let x: i32 = 3;
    let y: i32 = x + 12;
    let z: i32 = x + 12 - y;
    //%assert a == 0
    //%assert x == 3
    //%assert z == y - 15
    //%assert a + xdd == 13
}

fn array_prove() {
    let mut x: [i32; 3] = [1, 2, 3];
    //%assert x[0] == 1
    //%assert x[0] + x[1] == 3

    // TODO:
    //x[1] = 3;
    ////%assert x[1] == 3
}
