/*
	Static Variables
		Static variables are variables that are associated with a particular module or function and are accessible from anywhere in the program.
		Static variables are defined using the static keyword.
		Static variables are defined using the static keyword.

		The static keyword is used to define a static variable.

		Ways to use static variables in RUST:
			- Immutable Static Variables: Use them for constants or values that don’t need to change over the program’s lifetime, like configuration values.
				static PI: f64 = 3.1415;

			- Mutable Static Variables with unsafe: Use them when you need to modify a global value, but be cautious and use the unsafe block to manage safety yourself.
				static mut COUNT: i32 = 0;
			
			- Mutex-Wrapped Static Variables: Use Mutex (or RwLock for read-write scenarios) when you need to modify a global value in a thread-safe manner.
				static VALUE: Mutex<i32> = Mutex::new(0);
			
			- Atomic Static Variables: Use atomic types (AtomicUsize, AtomicBool, etc.) for lock-free, thread-safe operations when high performance is crucial.
				static FLAG: AtomicBool = AtomicBool::new(false);
*/

use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

// Simple Immutable Static Variable
static GREETING: &str = "Hello World...!";
// Mutable Static Variable with unsafe Block
static mut COUNTER: i32 = 0;
// Using Mutex for Safe Mutable Static Variables
static _COUNTER: Mutex<i32> = Mutex::new(0);
// Using Atomic Types for Concurrent Access
static COUNTER_: AtomicU32 = AtomicU32::new(0);

fn main() {
	// Simple Immutable Static Variable
	println!("{}", GREETING);

	// Mutable Static Variable with unsafe Block
	unsafe {
		COUNTER += 1;
		println!("Counter: {}", COUNTER);
	}

	// Using Mutex for Safe Mutable Static Variables
	{
		let mut counter = _COUNTER.lock().unwrap();
		*counter += 1;
		*counter += 1;
		println!("Counter: {}", *counter);
	}

	{
		let mut counter = _COUNTER.lock().unwrap();
		*counter += 1;
		println!("Counter: {}", *counter);
	}

	match _COUNTER.lock() {
		Ok(counter) => println!("Counter: {}", *counter),
		Err(e) => println!("Error: Mutex Lock Failed {}", e),
		_ => panic!("Error: Mutex Lock Failed")
	}

	println!("Counter: {}", COUNTER_.load(Ordering::SeqCst));

	// Using Atomic Types for Concurrent Access
	COUNTER_.fetch_add(1, Ordering::SeqCst);
	println!("Counter: {}", COUNTER_.load(Ordering::SeqCst));

	COUNTER_.fetch_add(1, Ordering::SeqCst);
	println!("Counter: {}", COUNTER_.load(Ordering::SeqCst));
	println!("Counter: {}", COUNTER_.load(Ordering::SeqCst));

}