#[derive(Debug)]
enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}
use Shape::*;

fn area(s: Shape) -> f32 {
    match s {
        Circle(r) => 3.14 *  ,
        Square(x) => ,
        Rectangle() => ,
    }
}

fn main() {
    let s = Rectangle(10.0, 20.0);
    println!("{:?}", area(s));
}
