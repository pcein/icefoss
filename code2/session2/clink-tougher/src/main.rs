
extern {
    fn c_add(x:i32, y:i32) -> i32;
}

fn main() {
    let r = unsafe {c_add(1,2)};
    println!("r = {}", r);
}
