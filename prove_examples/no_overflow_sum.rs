//%precondition _half_int_high == 1073741823 && _half_int_low == -1073741823 && x < _half_int_high && x > _half_int_low && y < _half_int_high && y > _half_int_low
//%postcondition return_value == x + y
fn no_overflow_sum(mut x: i32, mut y: i32) -> i32 {
    (x + y)
}

fn main() {}
