fn change(t1: &mut Vec<i32>) {
    t1[0] = 10;
}
fn main() {
    let mut v = vec![1,2,3];
    let p = &v; // an immutable borrow
    change(&mut v);
    println!("{:?}", v);
}
