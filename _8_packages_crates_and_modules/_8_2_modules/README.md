# Modules

## Key Concepts of Rust Modules
- Modules can contain other modules, functions, structs, enums, constants, and more.
- Module system allows you to nest modules for better Organization.
- `mod` keyword defines a module.
- `use` keyword allows you to import and use items from other modules.
- public (`pub`) and private access control the visibility of items inside a module.

### Basic Module System (Single File)
- In a single file (`main.rs` or `lib.rs`), modules can be defined using the mod keyword.
	```rust
	// main.rs

	mod math_utils {
		// This function is private by default
		fn add(a: i32, b: i32) -> i32 {
			a + b
		}

		// Make this function public with `pub`
		pub fn multiply(a: i32, b: i32) -> i32 {
			a * b
		}
	}

	fn main() {
		// Can't call private `add()` function
		// let sum = math_utils::add(2, 3);  // This will cause an error

		// Call the public `multiply()` function
		let product = math_utils::multiply(2, 3);
		println!("Product: {}", product);
	}
	```

### Modules with Multiple Files
- When your codebase grows, you may want to separate modules into their own files or directories for better organization. Rust allows you to split module definitions across multiple files.

	- Let's split the module math_utils into its own file.

		```
		my_project/
		├── Cargo.toml
		└── src/
			├── main.rs
			└── math_utils.rs

		```
	
	- `main.rs`
		```rust
		mod math_utils;  // Declaring that math_utils is in another file

		fn main() {
			let product = math_utils::multiply(2, 3);
			println!("Product: {}", product);
		}

		```

	- `math_utils.rs`
		```rust
		pub fn multiply(a: i32, b: i32) -> i32 {
			a * b
		}
		```
- Here, the mod math_utils; line in `main.rs` tells Rust to look for the module in the `math_utils.rs` file. Since multiply is public, it can be used in `main.rs`.

### Nested Modules
- Modules can be nested inside other modules, creating a hierarchy. You can also split nested modules across directories.

	- Nested Modules in One File
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

		fn main() {
			let sum = math_utils::arithmetic::add(2, 3);
			let square = math_utils::algebra::square(4);

			println!("Sum: {}", sum);
			println!("Square: {}", square);
		}
		```
	- math_utils has two nested modules: `arithmetic` and `algebra`.
	- Each module has its own public functions that can be accessed with the full path:
		- `math_utils::arithmetic::add and math_utils::algebra::square`. 

### Nested Modules in Separate Files and Directories

- When you have many nested modules, it’s common to organize them into directories.

	- File Structure

	```
	my_project/
	├── Cargo.toml
	└── src/
		├── main.rs
		└── math_utils/
			├── mod.rs
			├── arithmetic.rs
			└── algebra.rs
	```

	- `main.rs`
		```rust
		mod math_utils;  // Declare the `math_utils` module, located in the `math_utils` directory

		fn main() {
			let sum = math_utils::arithmetic::add(2, 3);
			let square = math_utils::algebra::square(4);

			println!("Sum: {}", sum);
			println!("Square: {}", square);
		}

		```
	
	- `math_utils/mod.rs`
		```rust
		pub mod arithmetic;  // Declare the arithmetic module
		pub mod algebra;     // Declare the algebra module
		```
	
	- `math_utils/arithmetic.rs`
		```rust
		pub fn add(a: i32, b: i32) -> i32 {
			a + b
		}
		```
	
	- `math_utils/algebra.rs`
		```rust
		pub fn square(x: i32) -> i32 {
			x * x
		}
		```
	
	- `main.rs` declares mod `math_utils`, which tells Rust to look for the module in a directory called `math_utils/`.
	- Inside the `math_utils/` directory, `mod.rs` serves as the entry point for the module. It declares the arithmetic and algebra submodules.
	- The submodules are then placed in their respective files: `arithmetic.rs` and `algebra.rs`.

### Re-exporting Items
- Sometimes, you may want to simplify the access path to certain functions or types by re-exporting them.

	- Re-Exporting with `pub use`
		```rust
		mod math_utils {
			pub mod arithmetic {
				pub fn add(a: i32, b: i32) -> i32 {
					a + b
				}
			}

			// Re-export the `add` function directly from `math_utils`
			pub use arithmetic::add;
		}

		fn main() {
			// Instead of using the full path `math_utils::arithmetic::add`, you can directly use `math_utils::add`
			let sum = math_utils::add(2, 3);
			println!("Sum: {}", sum);
		}
		```
		- Here, pub use arithmetic::add; re-exports the add function, allowing it to be accessed directly from math_utils.

### Using `use` for Shorter Paths

- To avoid writing long paths when accessing functions from deeply nested modules, you can use the use keyword to bring modules or items into scope.

	- Using use to Shorten Paths
		```rust
		mod math_utils {
			pub mod arithmetic {
				pub fn add(a: i32, b: i32) -> i32 {
					a + b
				}
			}
		}

		use math_utils::arithmetic::add;

		fn main() {
			let sum = add(2, 3);  // Now you can call `add` directly
			println!("Sum: {}", sum);
		}
		```
	- `use math_utils::arithmetic::add;` brings the `add` function into scope, so you don't need to write the full path every time.

### Private Modules and Functions

-  By default, all functions, structs, and modules are private unless explicitly made public using the `pub` keyword. This ensures that you can control access to your internal code.

	- `Private` vs `Public` Functions
		```rust
		mod math_utils {
			fn private_function() {
				println!("This is private!");
			}

			pub fn public_function() {
				println!("This is public!");
			}
		}

		fn main() {
			// math_utils::private_function();  // This would cause an error, as it's private
			math_utils::public_function();      // This is fine, as it's public
		}

		```
	- In this case, private_function cannot be called from main() because it's private to the math_utils module.

### Conditional Compilation
- Rust allows you to compile code based on certain conditions. This is useful when you want to include or exclude certain code based on the target platform or other factors.
- Conditional compilation is done using the `#[cfg()]` attribute.
- The `#[cfg()]` attribute takes a condition as an argument. The condition can be a simple boolean expression or a more complex expression that evaluates to a boolean value.
- The condition can be a simple boolean expression, such as `cfg(target_os = "windows")`, which checks if the target operating system is Windows.
- The condition can be a more complex expression, such as `cfg(target_os = "windows") && cfg(target_arch = "x86_64")`, which checks if the target operating system is Windows and the target architecture is x86_64.
- The condition can also be a more complex expression, such as `cfg(target_os = "windows") && cfg(target_arch = "x86_64") && cfg(feature = "some_feature")`, which checks if the target operating system is Windows, the target architecture is x86_64, and the feature "some_feature" is enabled.

	```rust
	#[cfg(target_os = "linux")]
	mod linux_only {
		pub fn linux_function() {
			println!("This only compiles on Linux");
		}
	}

	#[cfg(not(target_os = "linux"))]
	mod non_linux {
		pub fn other_function() {
			println!("This compiles on non-Linux systems");
		}
	}
	```

## Summary of Module Concepts:
- **Modules** are defined using the `mod` keyword and can contain nested modules, functions, structs, and more. - 
- **Public and private** visibility controls access to module items.
- **Single file or multiple files:** Modules can either live in the same file as the main code or be split into separate files or directories.
- `use` keyword allows you to bring items into scope to avoid writing full paths.
- **Re-exporting** items with `pub use` simplifies module access paths.
- **Private items** (functions, structs) are inaccessible from outside the module unless declared public (`pub`).
- This modular system allows you to organize large Rust codebases cleanly and intuitively.