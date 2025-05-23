fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let x = 10;
    let y = 0;
    match divide(x, y) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero!"),
    }
}
