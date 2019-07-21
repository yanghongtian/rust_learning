fn main() {
    println!("Hello, world!");

    another_function(100, 0.876);

    let x = five();
    println!("x = {}", x);

    let x = plus_one(x);
    println!("x + 1 = {}", x);
}


fn another_function(x: i32, y:f32) {
    println!("Another function!");
    println!("x = {}, y = {}", x, y);
}

fn five() -> i32 {
    // return 5;
    5
}


fn plus_one(x: i32) -> i32 {
    x + 1
}
