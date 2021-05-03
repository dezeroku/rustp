//%precondition x >= 0 && y >= 0
//%postcondition quo * y + rem == x
fn remainder(x: i32, y: i32) -> (i32, i32) {
    // First program proved by Hoare
    let mut quo: i32 = 0;
    let mut rem: i32 = x;

    // No variant, as this program is only partially correct
    // If y >= 0 then it's possible for rem to loop infinitely
    //%invariant quo * y + rem == x
    while rem >= y {
        rem = rem - y;
        quo = quo + 1;
    }

    (quo, rem)
}

fn main() {}
