// missing "mut"; also a missing line
fn factorial(n: i32) -> i32 {
    let f = 1;
    while n > 0 {
        f = f * n;
        // missing line
    }
    f
}
fn main() {
    let r = factorial(4);
    println!("{}", r);
}
