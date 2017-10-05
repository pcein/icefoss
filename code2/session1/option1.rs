
fn main() {
    let v1:Option<i32> = None;
    let v2:Option<i32> = Some(10);

    match v1 {
        None => println!("Option value is None"),
        Some(x) => println!("x = {}", x),
    }
    match v2 {
        None => println!("Option value is None"),
        Some(x) => println!("x = {}", x),
    }
}
