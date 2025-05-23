mod math_utils {
	#[allow(dead_code)]	
	pub fn private_function() {
		println!("This is private!");
	}

	pub fn public_function() {
		println!("This is public!");
	}
}

fn main() {
	math_utils::private_function();  // This would cause an error, as it's private
	math_utils::public_function();      // This is fine, as it's public
}
