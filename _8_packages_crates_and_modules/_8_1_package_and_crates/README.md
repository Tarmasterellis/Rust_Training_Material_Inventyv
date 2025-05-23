# Packasges and Crates

## Crates
- A crate is the smallest unit of code distribution in Rust. Crates can be either a binary crate or a library crate.

	### Binary Crates
	- Binary Crates produce an executable file.

		- Create a new binary crate with Cargo (Rust's package manager and build tool):

			```
			cargo new my_binary_crate
			```
		- This generates a new project with the following structure:

			```
			my_binary_crate
			├── Cargo.toml
			└── src
				└── main.rs
			```
		- The src/main.rs file is the entry point of the program:

			```rust
			fn main() {
				println!("Hello, world!");
			}
			```
		
		- Build and run the binary:
			```
			cargo run
			```

	### Library Crates
	- Library Crates are reusable code that can be included in other projects as dependencies.

		- Create a new library crate:
			```
			cargo new my_library_crate --lib
			```
		- The directory structure will look like this:

			```
			my_library_crate
			├── Cargo.toml
			└── src
				└── lib.rs
			```
		
		- The src/lib.rs file contains the library's public API:
			```rust
			pub fn greet() {
				println!("Hello from the library!");
			}
			```
		
		- You can build this crate using cargo build, and it can be included as a dependency in other projects.

			- Example 1.3: Using a Library Crate in a Binary Crate
				- Let's say you have a binary crate my_binary_crate and you want to use the my_library_crate as a dependency.

				- Add the library crate as a dependency in my_binary_crate's Cargo.toml:

					```toml
						[dependencies]
						my_library_crate = { path = "../my_library_crate" }
					```
			- In my_binary_crate/src/main.rs, use the library crate:

				```rust
				use my_library_crate::greet;

				fn main() {
					greet();  // Calls the function from the library
				}
				```

			- Run the binary crate, and it will invoke the library function:

			```
			cargo run
			```

## Packages
- A package is a bundle that can contain one or more crates (binary or library). Every package has a Cargo.toml file that defines its dependencies, configuration, and the crates it contains.

	- A package must contain at least one crate.
	- A package can contain at most one library crate.
	- A package can contain multiple binary crates.

	### A Package with Multiple Crates
	- Create a new package with a library and a binary:
		```
		cargo new my_package --lib
		```
	
	- this creates a structure like:
		```
		my_package
		├── Cargo.toml
		└── src
			└── lib.rs
		```
	
	- Now add a binary crate to this package:
		```rust
		fn main() {
			println!("This is the binary part of the package!");
		}
		```
	
	- Now the package contains:

		- A library crate (src/lib.rs)
		- A binary crate (src/main.rs)
	
	- You can compile the binary crate with:
		```
		cargo run
		```
	
	### A Package with Multiple Binary Crates
	- Add multiple binary crates by creating a src/bin directory:
		```
		my_package
		├── Cargo.toml
		└── src
			├── lib.rs
			└── bin
				├── bin1.rs
				└── bin2.rs
		```
	
	- Each file in the src/bin directory is treated as a separate binary crate. For example:
		```rust
		// src/bin/bin1.rs
		fn main() {
			println!("This is binary 1!");
		}
		```
		```rust
		// src/bin/bin2.rs
		fn main() {
			println!("This is binary 2!");
		}
		```
	
	- To run these binaries, use:
		```
		cargo run --bin bin1
		cargo run --bin bin2
		```

## Creating and Publishing Crates
- If you want to share your crate with others, you can publish it on crates.io.

- Steps to Publish a Crate:
	
	- Ensure your crate is ready:

		- Make sure the Cargo.toml file has necessary fields like name, version, authors, and description.

		- Run cargo test to ensure everything works.
	
	- Log in to crates.io:
		```
		cargo login
		```
	
	- Publish the crate:
		```
		cargo publish
		```

- Once published, others can add your crate to their projects by specifying it in their Cargo.toml file.

- Example 3.1: Using an External Crate from crates.io

	- Add the dependency in your Cargo.toml:

		```toml
		[dependencies]
		rand = "0.8"  # Using the `rand` crate for random number generation
		```
	- Use the crate in your project:

		```rust
		use rand::Rng;

		fn main() {
			let mut rng = rand::thread_rng();
			let n: u32 = rng.gen_range(1..101);
			println!("Random number: {}", n);
		}
		```
	- Run the project:
		```
		cargo run
		```
