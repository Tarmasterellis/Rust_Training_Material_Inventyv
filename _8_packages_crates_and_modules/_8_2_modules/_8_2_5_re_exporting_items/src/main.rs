mod math_utils {
	pub mod arithmetic {
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}
	}

	// Re-export the `add` function directly from `math_utils`
	pub use arithmetic::add;
}

fn main() {
	// Instead of using the full path `math_utils::arithmetic::add`, you can directly use `math_utils::add`
	let sum = math_utils::add(2, 3);
	println!("Sum: {}", sum);
}
