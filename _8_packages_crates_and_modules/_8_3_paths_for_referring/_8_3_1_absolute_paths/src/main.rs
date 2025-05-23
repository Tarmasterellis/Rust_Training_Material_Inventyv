mod math_utils {
	pub fn add(a: i32, b: i32) -> i32 {
		a + b
	}

	pub mod algebra {
		pub fn square(x: i32) -> i32 {
			x * x
		}
	}
}

fn main() {
	// Using an absolute path starting from `crate` (crate root)
	let sum = crate::math_utils::add(5, 3);
	let squared = crate::math_utils::algebra::square(4);

	println!("Sum: {}, Square: {}", sum, squared);
}
