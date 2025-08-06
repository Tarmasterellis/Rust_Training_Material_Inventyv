/*
	What is a Closure in Rust?
		- A closure in Rust is an anonymous function that can capture variables from its surrounding scope. Closures are useful for short, reusable operations.

		- Closures in Rust can be written in different ways:
			- Basic Closure (Short function-like syntax)
			- Capturing Variables (By reference, mutable reference, or by value)
			- Returning Closures (Using impl Fn or Box<dyn Fn> due to type inference limitations)
			- Using Closures in Higher-Order Functions (map, filter, etc.)
			- Closures with Different Traits (Fn, FnMut, FnOnce)

	

*/


fn main() {
	// 1. Basic Closure (Inline syntax, similar to a normal function)
	let add = |a: i32, b: i32| a + b;
	println!("Sum using closure: {}", add(5, 10)); // Output: 15

	// 2. Capturing Variables from the Environment
	let multiplier = 3;
	
	// Closure capturing 'multiplier' by **reference**
	let multiply_by = |x: i32| x * multiplier;
	println!("Multiply using closure: {}", multiply_by(4)); // Output: 12

	// 3. Mutable Capture - Closure modifying external variable
	let mut count = 0;
	let mut increment = || count += 1;
	
	increment();
	increment();
	println!("Counter after closure: {}", count); // Output: 2

	// 4. Moving Ownership into Closure (FnOnce Trait)
	let my_string = String::from("Rust");
	let consume_string = move || println!("Captured: {}", my_string);
	
	// my_string is **moved** and cannot be used after this point
	consume_string();
	
	// 5. Returning a Closure (Requires 'impl Fn')
	fn returns_closure() -> impl Fn(i32) -> i32 { |x| x * 2 }
	
	let double = returns_closure();
	println!("Double using closure: {}", double(6)); // Output: 12

	// 6. Using Closure in Higher-Order Functions (map, filter)
	let numbers = vec![1, 2, 3, 4, 5];
	
	// Using closure in map
	let squared_numbers: Vec<i32> = numbers.iter().map(|x| x * x).collect();
	println!("Squared Numbers: {:?}", squared_numbers); // Output: [1, 4, 9, 16, 25]

	// Using closure in filter
	let even_numbers: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
	println!("Even Numbers: {:?}", even_numbers); // Output: [2, 4]

	// 7. Using Fn, FnMut, FnOnce Explicitly
	fn execute<F: Fn(i32)>(f: F, num: i32) {
		f(num);
	}

	let print_num = |x: i32| println!("Fn Trait Closure: {}", x);
	execute(print_num, 100); // Output: 100
}
