mod math_utils {
	fn base_value() -> i32 {
		10
	}

	pub mod arithmetic {
		// Use `super` to access the parent module's `base_value` function
		pub fn add_to_base(x: i32) -> i32 {
			super::base_value() + x
		}
	}
}

fn main() {
	let result = math_utils::arithmetic::add_to_base(5);
	println!("Result: {}", result);
}
