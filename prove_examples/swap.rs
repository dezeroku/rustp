//%precondition true
//%postcondition x == y'old && y == x'old
fn swap(mut x: i32, mut y: i32) -> (i32, i32) {
    let t: i32 = x;
    x = y;
    y = t;
    (x, y)
}

fn main() {}
