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

fn main() {
	let sum = math_utils::arithmetic::add(2, 3);
	let square = math_utils::algebra::square(4);

	println!("Sum: {}", sum);
	println!("Square: {}", square);
}
