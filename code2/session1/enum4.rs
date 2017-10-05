
#[derive(Debug)]
enum MyOption <T>{
    MyNone,
    MySome(T),
}

use MyOption::*;

fn main() {
    let x = MySome(10);
    let y = MySome("hello");
    let z:MyOption<i32> = MyNone;

    println!("x={:?},y={:?},z={:?}", x, y, z);
}
