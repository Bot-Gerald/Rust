use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let int_result = add(5, 10);
    let float_result = add(5.5, 10.5);

    println!("Integer addition result: {}", int_result);
    println!("Float addition result: {}", float_result);
}
