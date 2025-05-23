# Error Handling

- Error handling in Rust is a key feature that ensures safety and robustness in programs. Rust uses two primary types for error handling:

	1. Result<T, E>: Used for recoverable errors.
	2. panic!: Used for unrecoverable errors.

1. Result<T, E>: Recoverable Errors
- The Result type is used for functions that might return an error. It has two variants:

	- Ok(T) — Represents a success, containing the result value.
	- Err(E) — Represents an error, containing the error value.

- The general signature of Result is:

	```rust
	enum Result<T, E> {
		Ok(T),
		Err(E),
	}
	```

- Basic Example
	- Here’s how Result is typically used:

		```rust
		fn divide(a: i32, b: i32) -> Result<i32, String> {
			if b == 0 {
				Err(String::from("Cannot divide by zero"))
			} else {
				Ok(a / b)
			}
		}

		fn main() {
			match divide(4, 2) {
				Ok(result) => println!("Result: {}", result),
				Err(e) => println!("Error: {}", e),
			}

			match divide(4, 0) {
				Ok(result) => println!("Result: {}", result),
				Err(e) => println!("Error: {}", e),
			}
		}
		```

1. Handling Result<T, E>
	- There are several ways to handle the result:
		1. match expression: This is the most explicit way, as shown in the example above.
		
		2. unwrap(): This will return the value inside Ok, or panic if it's an Err.

			```rust
			fn main() {
				let result = divide(4, 2).unwrap();
				println!("Result: {}", result);  // This will work
				
				let result = divide(4, 0).unwrap();  // This will panic at runtime
			}
			```
		
		3. unwrap_or(): Provides a default value in case of an error.
			```rust
			fn main() {
				let result = divide(4, 0).unwrap_or(0);
				println!("Result: {}", result);  // Prints 0
			}
			```
		
		4. unwrap_or_else(): Allows handling errors by providing a closure that generates a value.
			```rust
			fn main() {
				let result = divide(4, 0).unwrap_or_else(|_| -1);
				println!("Result: {}", result);  // Prints -1
			}
			```
		
		5. ? Operator: This is a shorthand for propagating errors. It returns the error to the calling function if the result is Err, or continues execution if the result is Ok.
			```rust
			fn divide(a: i32, b: i32) -> Result<i32, String> {
				if b == 0 {
					return Err(String::from("Cannot divide by zero"));
				}
				Ok(a / b)
			}

			fn calculate() -> Result<(), String> {
				let result = divide(10, 2)?;  // Automatically returns error if it occurs
				println!("Result: {}", result);
				Ok(())
			}

			fn main() {
				if let Err(e) = calculate() {
					println!("Error: {}", e);
				}
			}
			```

2. panic!: Unrecoverable Errors
- The panic! macro is used when a program encounters an unrecoverable error. When panic! is called, the program will terminate, unwind the stack, and clean up any resources.

	- Basic Example of panic!
		```rust
		fn main() {
			let v = vec![1, 2, 3];

			// This will panic because we're accessing out of bounds
			println!("{}", v[99]);
		}
		```

- By default, Rust will unwind the stack during a panic, meaning that it will clean up and free memory for variables that were created. You can also opt for an immediate program abort by setting the panic behavior to "abort" in your Cargo.toml.

- When to Use panic!
	- Program Logic Error: If the error represents a bug in the program logic and there's no point in trying to recover.
	- Prototyping: While writing the first draft of a program, it can be convenient to use panic! to handle unimplemented features.

3. Custom Error Types
- Sometimes it is useful to create custom error types, especially when handling errors in larger programs. You can use enums to define multiple error cases.

	- Example: Custom Error Enum
		```rust
		use std::fmt;

		#[derive(Debug)]
		enum MathError {
			DivisionByZero,
			NegativeLogarithm,
		}

		// Implement Display for custom error types
		impl fmt::Display for MathError {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				match *self {
					MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
					MathError::NegativeLogarithm => write!(f, "Logarithm of a negative number"),
				}
			}
		}

		fn divide(a: i32, b: i32) -> Result<i32, MathError> {
			if b == 0 {
				Err(MathError::DivisionByZero)
			} else {
				Ok(a / b)
			}
		}

		fn main() {
			match divide(10, 0) {
				Ok(result) => println!("Result: {}", result),
				Err(e) => println!("Error: {}", e),
			}
		}
		```

4. Option<T>: Handling Absence of Value
- Option<T> is another important type for handling cases where a value might be absent (but not necessarily an error). It has two variants:

	- Some(T) — Contains a value.
	- None — Indicates the absence of a value.

	- Example with Option<T>
		```rust
		fn find_word(sentence: &str, word: &str) -> Option<usize> {
			sentence.find(word)  // Returns Some(index) or None
		}

		fn main() {
			match find_word("Hello world", "world") {
				Some(index) => println!("Found at index: {}", index),
				None => println!("Word not found"),
			}
		}
		```

- Combining Result<T, E> and Option<T>
	- Sometimes Option and Result are used together. For example, a function might return an Option inside a Result to signal both errors and the absence of a value.

		```rust
		fn find_word_safe(sentence: &str, word: &str) -> Result<Option<usize>, String> {
			if sentence.is_empty() {
				return Err(String::from("Empty sentence"));
			}
			Ok(sentence.find(word))  // Some(index) or None
		}

		fn main() {
			match find_word_safe("Hello world", "world") {
				Ok(Some(index)) => println!("Found at index: {}", index),
				Ok(None) => println!("Word not found"),
				Err(e) => println!("Error: {}", e),
			}
		}
		```


## Summary of Key Concepts
1. Result<T, E>: For recoverable errors.
	- Use Ok(T) for success.
	- Use Err(E) for failure.
	- Use ? to propagate errors.

2. panic!: For unrecoverable errors.
	- The program will terminate.
	- Use when encountering a bug or in prototyping.

3. Option<T>: For cases where a value might be absent, but not due to an error.

4. Custom Errors: Create your own error types using enum and implement fmt::Display for more detailed error messages.

- **Note:** By following these patterns, you can effectively manage both recoverable and unrecoverable errors in Rust, resulting in robust and maintainable code.