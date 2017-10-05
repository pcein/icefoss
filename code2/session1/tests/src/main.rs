
#[test]
fn sqr_0_test() {
    assert_eq!(sqr(0), 0);
}

#[test]
fn sqr_neg_test() {
    assert_eq!(sqr(-1), 1);
}

#[test]
fn sqr_positive_test() {
    assert_eq!(sqr(2), 4);
}

fn sqr(x: i32) -> i32 {
    x * x 
}

fn main() {
    println!("Hello, world!");
}
