mod math_utils {
	pub mod arithmetic {
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}
	}

	pub mod algebra {
		pub fn multiply(a: i32, b: i32) -> i32 {
			a * b
		}
	}
}

fn main() {
	// Accessing items in sibling modules using absolute paths
	let sum = crate::math_utils::arithmetic::add(2, 3);
	let product = crate::math_utils::algebra::multiply(2, 3);

	println!("Sum: {}, Product: {}", sum, product);
}
