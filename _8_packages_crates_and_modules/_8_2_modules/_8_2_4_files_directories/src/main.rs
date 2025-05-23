mod math_utils::arithmetic::add;  // Declare the `math_utils` module, located in the `math_utils` directory

fn main() {
	let sum = math_utils::arithmetic::add(2, 3);
	let square = math_utils::algebra::square(4);

	println!("Sum: {}", sum);
	println!("Square: {}", square);
}
