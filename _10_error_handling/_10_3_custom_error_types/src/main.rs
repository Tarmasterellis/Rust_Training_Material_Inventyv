/* Error Handling using custom error types */

use std::fmt;

#[derive(Debug)]
enum MathError {
	DivisionByZero,
	NegativeLogarithm,
}

// Implement Display for custom error types
impl fmt::Display for MathError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
			MathError::NegativeLogarithm => write!(f, "Logarithm of a negative number"),
		}
	}
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
	if b == 0 {
		Err(MathError::DivisionByZero)
	}
	else if b < 0 {
		Err(MathError::NegativeLogarithm)
	}
	else {
		Ok(a / b)
	}
}

fn main() {
	match divide(10, 0) {
		Ok(result) => println!("Result: {}", result),
		Err(e) => println!("Error: {:?}", e),
	}
}
