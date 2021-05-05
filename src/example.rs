fn overflow() {
    let max: i32 = 10000000;
    let i: i32 = 100000 * 100000000 * 1000000000;
    let j: i32 = max * i;
    //%assert j == i * max
}

//%precondition n > 0
//%postcondition (forall x !(x >= 0 && x < n) || max >= a[x]) && (exists x x >= 0 && x < n && max == a[x])
fn max(a: &[i32], n: usize) -> i32 {
    let mut max: i32 = a[0];
    let mut i: usize = 1;
    let t: i32 = -1;

    //%invariant (forall y !(y >= 0 && y < i) || max >= a[y]) && (exists x x >= 0 && x < i && max == a[x]) && i <= n && n > 0
    while i < n {
        if max < a[i] {
            max = a[i];
        }
        i = i + 1;
    }
    max
}

fn test4(mut a: i32) {
    let mut i: i32 = 10;
    //%invariant i >= 0
    //%variant i
    while i > 0 {
        i = i - 1;
    }
}

fn test3(mut a: i32) {
    a = 123;
    let mut t: [i32; 3] = [0, 0, 0];
    //t = [1, 2, 3];
    t[1] = 2;
    let b: i32 = 1;
    //%assert t[1] == 2
}

fn test() {
    let mut t: [i32; 3] = [0, 0, 0];
    //t = [1, 2, 3];
    t[1] = 2;
    let b: i32 = 1;
    //%assert t[1] == 2
}

fn test2() {
    let mut t: [i32; 3] = [0, 0, 0];
    //t = [1, 2, 3];
    t[1] = 2;
    t[0] = 4;
    t[1] = 2;
    let b: i32 = 1;
    //%assert t[b] == 2
    //%assert exists i t[i] > 0
}

fn dummy() {
    let mut q: i32 = 0;
    let r: i32 = 2;
    let mut z: i32 = 1;
    //%invariant q + r == 2 * z
    while z >= q {
        z = z + 1;
        q = q + 2;
    }
}

//%precondition x == 2
//%postcondition x == 4 && x == x'old + 2 && return_value == 12
fn temp(mut x: i32) -> i32 {
    // TODO: seems that we don't do anything with the return_value at the moment?
    x = x + 2;
    if 12 > 3 {
        x = x + 0;
    } else {
        x = x + 13;
    }
    (x + 8)
}

//%precondition x == 2
//%postcondition x == 4 && x == x'old + 2 && return_value == 12
fn temp_if(mut x: i32) -> i32 {
    let y: bool = false;
    if !y {
        x = x + 1;
        if 12 < 3 {
            x = x + 3;
        } else {
            if 3 < 2 {
                x = x + 3;
            } else {
                x = x + 1;
            }
        }
    } else {
        x = x + 12;
    }
    (x + 8)
}

//%precondition x>=0 && y >=0
//%postcondition quo * y + rem == x
fn remainder_simple(x: i32, y: i32) {
    //let x: i32 = 27;
    //let y: i32 = 4;

    // First program, proved by Hoare
    let mut quo: i32 = 0;
    let mut rem: i32 = x;

    //%invariant quo * y + rem == x
    while rem >= y {
        rem = rem - y;
        quo = quo + 1;
    }

    ////%assert quo * y + rem == x

    //%debug
    println!("{} {}: {} {}", x, y, quo, rem);

    // End of the first program
}

//%precondition x == 3
fn easy(mut x: i32) {
    x = x + 1;
    //%assert x == 4
    //%assert x'old == 3

    x = 2;
    //%assert x == 2

    //x = x + 1;
    ////%assert x == 5

    //let mut a: i32 = 13;
    // %assert a == 13

    //a = 123;
    //a = 123;
    //a = 123;
    ////%assert a != 123
}

fn main() {
    //for i in 0..12 {}
    let a: i32 = 12;
    let b: i32 = 3 + a;

    // %assert a == 12

    //%debug
    println!("{}", b);

    //for i in 0..12 {}
    //%assert a == 12
    //%assert b == 15
    //%assert b == a + 3
    // %assert 15 > 13 && 7 == 7
}

//%precondition x >= 0 && y >= 0
fn remainder(x: i32, y: i32) -> (i32, i32) {
    // TODO: move this to the precondition

    // First program, proved by Hoare
    //let x: i32 = 27;
    //let y: i32 = 4;

    let mut quo: i32 = 0;
    let mut rem: i32 = x;

    //%invariant quo * y + rem == x
    while rem >= y {
        rem = rem - y;
        quo = quo + 1;
    }

    //%assert quo * y + rem == x

    //%debug
    println!("{} {}: {} {}", x, y, quo, rem);

    // End of the first program
    (quo, rem)
}

//%precondition a==0 && xdd == 13
fn c(a: i32, xdd: i32) {
    let x: i32 = 3;
    let y: i32 = x + 12;
    let mut z: i32 = x + 12 - y;
    //%assert a == 0
    //%assert x == 3
    //%assert z == y - 15
    z = 130;
    ////%assert z == 130
    //%assert a + xdd == 13
}

fn array_prove() {
    let mut x: [i32; 3] = [1, 2, 3];
    //x[0] = 1;
    //x[1] = 2;
    //x[2] = 3;
    let y: [i32; 3];
    //%assert x[0] == 1
    //%assert x[2] == 3
    //%assert x[0] + x[1] == 3
    //%assert x[1] == 2

    // TODO:
    x[1] = 3;
    //%assert x[1] == 3
}
