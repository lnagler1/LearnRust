fn main() {
    println!("Hello, world!");

    let x = five();

    println!("The value of x is: {}", x);
    another_function(x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
