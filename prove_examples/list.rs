//%precondition n >= 0
//%postcondition forall y !(y >= 0 && y < i) || x[y] == y
fn list(x: &mut [i32], n: usize) {
    let mut i: usize = 0;
    let mut j: i32 = 0;
    //%invariant i == j && n - i >= 0 && forall y !(y >= 0 && y < i) || x[y] == y
    //%variant n - i
    while i < n {
        x[i] = j;
        i = i + 1;
        j = j + 1;
    }
}

fn main() {}
