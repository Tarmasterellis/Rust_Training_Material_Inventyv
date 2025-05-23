mod math_utils {
	pub fn add(a: i32, b: i32) -> i32 {
		a + b
	}

	pub mod algebra {
		// use super::add;  // Using a relative path to refer to `add` in the parent module

		pub fn square_add(x: i32, y: i32) -> i32 {
			// Call the `add` function from the parent module using a relative path
			super::add(x, y) * super::add(x, y)
		}
	}
}

fn main() {
	let result = math_utils::algebra::square_add(2, 3);
	println!("Square of the sum: {}", result);
}