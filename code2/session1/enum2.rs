#[derive(Debug)]
enum Shape {
    Circle(i32),
    Square(i32),
    Rectangle(i32, i32),
}
use Shape::*;
fn main() {
    let s = Rectangle(10, 20);
    println!("{:?}", s);
}
