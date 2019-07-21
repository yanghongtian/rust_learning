
// constant definition, must specify constant type.
const MAX_POINTS: u32 = 100_100;
fn main() {
    // let mut x = 5;
    let x = 5;
    println!("x = {}", x);
    // compile error, can not assign twice to immutable variable
    x = 6;
    println!("x = {}", x);

    println!("the max points is: {}", MAX_POINTS);
}
