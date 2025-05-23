mod math_utils {
	pub mod arithmetic {
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}
	}

	pub mod algebra {
		pub fn square(x: i32) -> i32 {
			x * x
		}
	}
}

// Bringing the entire `arithmetic` module into scope
use math_utils::arithmetic;
use math_utils::algebra;

fn main() {
	// Now you can access `add` directly, without the full path
	let squared = algebra::square(arithmetic::add(4, 5));
	println!("Square: {}", squared);

	let square_add = arithmetic::add(4, 5) * arithmetic::add(4, 5);
	println!("Square of the sum: {}", square_add);
}
