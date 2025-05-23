# Paths for Referring

- In Rust, paths are used to refer to an item (such as functions, structs, enums, constants, or modules) in the module tree. Rust’s module system organizes code into a hierarchical structure, and paths specify the location of items within this hierarchy.

- There are two main types of paths in Rust:

	- Absolute paths: Start from the root of the crate.
	- Relative paths: Start from the current module.

- These paths allow you to access items from different parts of your code, whether they are in the same module, a different module, or an external crate.

## Absolute Paths
- An absolute path always starts from the crate root, which is the root of your Rust package (either `lib.rs` or `main.rs`). Absolute paths allow you to reference items from the crate's root, ensuring that you can access any item in any module.

	```rust
	mod math_utils {
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}

		pub mod algebra {
			pub fn square(x: i32) -> i32 {
				x * x
			}
		}
	}

	fn main() {
		// Using an absolute path starting from `crate` (crate root)
		let sum = crate::math_utils::add(5, 3);
		let squared = crate::math_utils::algebra::square(4);

		println!("Sum: {}, Square: {}", sum, squared);
	}

	```
- `crate::math_utils::add(5, 3)`: This is an absolute path that starts from the crate root (`crate`) and then goes into the `math_utils` module to access the `add` function.

- `crate::math_utils::algebra::square(4)`: This is another absolute path, referencing the `square` function within the `algebra` submodule.

## Relative Paths
- A relative path starts from the current module and allows you to navigate through sibling or parent modules without specifying the entire path from the crate root.

	- self refers to the current module.
	- super refers to the parent module.
	- If you don't specify a prefix, Rust assumes you're referring to an item in the current module.

	```rust
	mod math_utils {
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}

		pub mod algebra {
			use super::add;  // Using a relative path to refer to `add` in the parent module

			pub fn square_add(x: i32, y: i32) -> i32 {
				// Call the `add` function from the parent module using a relative path
				super::add(x, y) * super::add(x, y)
			}
		}
	}

	fn main() {
		let result = math_utils::algebra::square_add(2, 3);
		println!("Square of the sum: {}", result);
	}

	```

- `super::add`: This relative path refers to the `add` function in the parent module (`math_utils`).
- `use super::add`: Imports the `add` function from the parent module (`math_utils`) into the current module (`algebra`).

## Using use Keyword for Paths
- The use keyword allows you to bring items from other modules into the current scope, making it easier to reference them without writing the full path every time.

	```rust
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

	fn main() {
		// Now you can access `add` directly, without the full path
		let sum = arithmetic::add(4, 5);
		println!("Sum: {}", sum);
	}
	```
- `use math_utils::arithmetic;`: This brings the `arithmetic` module into the current scope, so you can refer to `add` using `arithmetic::add` rather than the full path `math_utils::arithmetic::add`.

## Re-exporting with pub use
- You can simplify the external API of your module by re-exporting items with pub use. This allows users of your module to access items with shorter paths.

	```rust
	mod math_utils {
		pub mod arithmetic {
			pub fn add(a: i32, b: i32) -> i32 {
				a + b
			}
		}

		// Re-exporting the `add` function for simpler access
		pub use arithmetic::add;
	}

	fn main() {
		// Access `add` directly from `math_utils`, without going through `arithmetic`
		let sum = math_utils::add(4, 5);
		println!("Sum: {}", sum);
	}
	```

- `pub use arithmetic::add;`: This re-exports the `add` function, allowing you to access it directly from `math_utils::add` instead of `math_utils::arithmetic::add`.

## Accessing Items in Sibling Modules
- You can use paths to access items in sibling modules, either through relative paths (`super`) or absolute paths (`crate`).

	```rust
	mod math_utils {
		pub mod arithmetic {
			pub fn add(a: i32, b: i32) -> i32 {
				a + b
			}
		}

		pub mod algebra {
			pub fn multiply(a: i32, b: i32) -> i32 {
				a * b
			}
		}
	}

	fn main() {
		// Accessing items in sibling modules using absolute paths
		let sum = crate::math_utils::arithmetic::add(2, 3);
		let product = crate::math_utils::algebra::multiply(2, 3);

		println!("Sum: {}, Product: {}", sum, product);
	}
	```

- `crate::math_utils::arithmetic::add(2, 3)`: An absolute path to access the `add` function in the `arithmetic` module.
- `crate::math_utils::algebra::multiply(2, 3)`: An absolute path to access the `multiply` function in the sibling `algebra` module.

## Paths with External Crates
- Paths are also used to refer to items from external crates. When you add a crate in Cargo.toml, you can use its items with absolute paths starting from the crate's name.

- Add the `rand` crate to your Cargo.toml:
	```toml
	[dependencies]
	rand = "0.8"
	```

- Now use it in your code:
	```rust
	// Import the `Rng` trait from the `rand` crate
	use rand::Rng;

	fn main() {
		// Create a random number generator
		let mut rng = rand::thread_rng();

		// Generate a random number
		let random_number: u32 = rng.gen_range(1..101);
		println!("Random number: {}", random_number);
	}
	```

- `rand::Rng`: This is an absolute path referring to the `Rng` trait in the `rand` crate.
- `rand::thread_rng()`: An absolute path to the `thread_rng` function in the `rand` crate.

## Paths to Items in the Same Module
- If you're accessing items from the same module, you don't need to specify self explicitly; you can refer to the item directly by name. However, you can use self for clarity.

	```rust
	mod math_utils {
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}

		pub fn multiply(a: i32, b: i32) -> i32 {
			self::add(a, b) * a  // Using `self` to refer to `add` in the same module
		}
	}

	fn main() {
		let product = math_utils::multiply(3, 2);
		println!("Product: {}", product);
	}
	```
- `self::add(a, b)`: Refers to the `add` function within the same module (`math_utils`).

## Relative Paths with super
- Use `super` to access items in a parent module from a child module.

	```rust
	mod math_utils {
		fn base_value() -> i32 {
			10
		}

		pub mod arithmetic {
			// Use `super` to access the parent module's `base_value` function
			pub fn add_to_base(x: i32) -> i32 {
				super::base_value() + x
			}
		}
	}

	fn main() {
		let result = math_utils::arithmetic::add_to_base(5);
		println!("Result: {}", result);
	}
	```

- `super::base_value()`: Refers to the `base_value` function in the parent module (`math_utils`).