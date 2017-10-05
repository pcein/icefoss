enum Color {
    Red,
    Green,
    Blue,
}

use Color::*;

fn main() {
    let c1 = Green; 
    // you can also write let c1:Color = Green

    // Bug here. Fix it!
    match c1 {
        Red    => println!("Red ..."),
        Green  => println!("Green ..."),
    }
}
    
