/*
	Lazy Static
		Lazy static variables are a way to define a static variable that is only initialized when it is first accessed.
		Lazy static variables are useful when you want to define a static variable that is only initialized when it is first accessed.
		Lazy static variables are defined using the lazy_static! macro.

		The lazy_static! macro is used to define a lazy static variable.

		The lazy_static! macro takes a block of code as an argument and executes it when the lazy static variable is first accessed.
		This ensures that the code is only executed once and the result is stored in the lazy static variable.

		To use lazy static variables, you need to import the lazy_static crate and use the lazy_static! macro.
*/

// Importing the lazy_static crate and Mutex
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
	// Defining a lazy static variable wrapped in a Mutex for safe mutability
	static ref COUNTER: Mutex<u32> = Mutex::new(0);
}

fn main() {



	// Lock the mutex to access the counter safely
	{
		let mut counter = COUNTER.lock().unwrap(); // Lock the mutex
		*counter += 1; // Increment the counter
		println!("Counter: {}", *counter); // Print the current counter value
	}

	{
		let mut counter = COUNTER.lock().unwrap(); // Lock the mutex again
		*counter += 1; // Increment the counter
		println!("Counter: {}", *counter); // Print the current counter value
	}

	println!("Counter: {:?}", COUNTER.lock().unwrap()); // Print the counter value again
}