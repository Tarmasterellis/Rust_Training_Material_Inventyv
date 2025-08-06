/* Error Handling using result */

fn main() {
    // Example of handling an error
    let result: Result<i32, String> = divide(4, 4);
    match result {
        Ok(result) => println!("Result: {:?}", result),
        Err(error) => println!("Error: {:?}", error),
    }



    // Example of handling error with unwrap()
    let result = divide(4, 2).unwrap();
    println!("Result: {:?}", result);

    // Example of handling error with unwrap_or_else()
    let result = divide(4, 0).unwrap_or_else(|_| -1);
    println!("Result: {:?}", result);

    // Example of handling error with ? operator
    if let Err(e) = calculate() {
        println!("Error: {}", e);
    }
}

// Function that divides two numbers
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}


fn calculate() -> Result<(), String> {
    let result = divide(10, 0)?;  // Automatically returns error if it occurs
    println!("Result: {}", result);
    Ok(())
}