fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let number = 5;
    let result = factorial(number);
    println!("The factorial of {} is {}", number, result);
}
