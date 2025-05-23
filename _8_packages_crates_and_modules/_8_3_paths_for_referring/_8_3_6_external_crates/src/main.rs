// Import the `Rng` trait from the `rand` crate
use rand::Rng;

fn main() {
	// Create a random number generator
	let mut rng = rand::thread_rng();

	// Generate a random number
	let random_number: u32 = rng.gen_range(1..101);
	println!("Random number: {}", random_number);
}
