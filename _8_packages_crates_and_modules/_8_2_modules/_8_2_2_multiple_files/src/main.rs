mod math_utils;  // Declaring that math_utils is in another file

fn main() {
	let product = math_utils::multiply(2, 3);
	println!("Product: {}", product);
}
