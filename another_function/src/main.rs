fn main() {
    println!("Hello, world!");

    another_function(five());
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn five() -> i32 {
    5
}