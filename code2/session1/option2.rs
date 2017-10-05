
fn main() {
    let v1:Option<i32> = None;
    let v2:Option<i32> = Some(10);

    println!("v2: x={}", v2.unwrap());
    println!("v1: x={}", v1.unwrap());
}
