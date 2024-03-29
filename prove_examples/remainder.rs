//%precondition x >= 0 && y > 0
//%postcondition quo * y + rem == x
fn remainder(x: i32, y: i32) -> (i32, i32) {
    // First program proved by Hoare
    let mut quo: i32 = 0;
    let mut rem: i32 = x;

    //%invariant quo * y + rem == x && y > 0 && rem >= 0
    //%variant rem
    while rem >= y {
        rem = rem - y;
        quo = quo + 1;
    }

    (quo, rem)
}

fn main() {}
