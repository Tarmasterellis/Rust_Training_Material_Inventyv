mod math_utils {
	pub mod arithmetic {
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}
	}

	// Re-exporting the `add` function for simpler access
	pub use arithmetic::add;
}

fn main() {
	// Access `add` directly from `math_utils`, without going through `arithmetic`
	let sum = math_utils::add(4, 5);
	println!("Sum: {}", sum);
}
