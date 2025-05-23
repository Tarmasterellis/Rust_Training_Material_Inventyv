mod math_utils {
	pub mod arithmetic {
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}
	}
}

use math_utils::arithmetic::add;

fn main() {
	let sum = add(2, 3);  // Now you can call `add` directly
	println!("Sum: {}", sum);
}
