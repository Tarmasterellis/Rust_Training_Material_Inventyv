// main.rs

mod math_utils {
	
	// This function is private by default
	#[allow(dead_code)]
	fn add(a: i32, b: i32) -> i32 {
		a + b
	}

	// Make this function public with `pub`
	pub fn multiply(a: i32, b: i32) -> i32 {
		a * b
	}
}

fn main() {
	// Can't call private `add()` function
	// let sum = math_utils::add(2, 3);  // This will cause an error

	// Call the public `multiply()` function
	let product = math_utils::multiply(2, 3);
	println!("Product: {}", product);
}
