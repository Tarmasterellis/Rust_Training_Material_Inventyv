mod math_utils {
	pub fn add(a: i32, b: i32) -> i32 {
		a + b
	}

	pub fn multiply(a: i32, b: i32) -> i32 {
		self::add(a, b) * a  // Using `self` to refer to `add` in the same module
	}
}

fn main() {
	let product = math_utils::multiply(3, 2);
	println!("Product: {}", product);
}
