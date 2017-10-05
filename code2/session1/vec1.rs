fn main() {
    let v1:Vec<i32> = vec![1,2,3];
    let mut v2 = vec![1.2, 2.3, 4.5];
    let v3 = vec!["cpp", "python", "perl"];

    v2.push(5);

    println!("v1 = {:?}, v2 = {:?}, v3 = {:?}", 
             v1, v2, v3);
    
}
