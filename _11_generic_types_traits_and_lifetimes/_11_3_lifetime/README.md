# Lifetimes

## Lifetimes

- Lifetimes in Rust are a way to ensure memory safety by enforcing that references are valid as long as they are needed. They describe the scope for which a reference is valid, preventing dangling references (references to data that has been deallocated). Lifetimes are an important feature in Rust’s ownership system, especially when working with multiple references.

- Lifetimes are not about the duration of the object in memory, but about ensuring that references are used correctly so that they don't outlive the data they point to.

- Rules of Lifetimes:
	- **Every reference has a lifetime**: The compiler tracks the scope in which references are valid and ensures they don't outlive the data they point to.
	- **Lifetimes are inferred**: In many cases, Rust is able to infer lifetimes automatically, especially for simple cases. But sometimes lifetimes must be explicitly specified.
	- **Lifetime annotations don’t change how long a reference lives**: They only tell the compiler how the lifetimes of different references relate to each other.
	- **Lifetime elision rules**: Rust has rules for eliding (omitting) lifetimes when the relationships are clear, reducing the need for explicit annotations in many cases.
	- **Multiple references with different lifetimes**: You can have multiple references with different lifetimes, but the compiler ensures they don’t overlap in ways that would cause errors.

- Lifetimes are a way to specify the lifetime of a reference. They are defined using apostrophes (`'`) and can be used in function signatures, struct definitions, and enum definitions.

	```rust
	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}

	struct Pair<'a> {
		first: &'a str,
		second: &'a str,
	}

	enum List<'a> {
		Empty,
		Cons(&'a str, Box<List<'a>>),
	}
	```

- In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create functions, structs, and enums that can work with different lifetimes.

## Lifetime Annotations

- Lifetime annotations are used to specify the lifetime of a reference. They are defined using apostrophes (`'`) and can be used in function signatures, struct definitions, and enum definitions.

	```rust
	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}

	struct Pair<'a> {
		first: &'a str,
		second: &'a str,
	}

	enum List<'a> {
		Empty,
		Cons(&'a str, Box<List<'a>>),
	}
	```

- In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create functions, structs, and enums that can work with different lifetimes.

## Lifetimes in Functions

- Lifetimes can be used in function signatures to create functions that can work with different lifetimes.

	```rust
	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}

	fn main() {
		let x = "hello";
		let y = "world";
		let longest = longest(x, y);
		println!("The longest string is: {}", longest);
	}
	```

- In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create functions that can work with different lifetimes.

## Lifetimes in Structs

- Lifetimes can be used in struct definitions to create structs that can work with different lifetimes.

	```rust
	struct Pair<'a> {
		first: &'a str,
		second: &'a str,
	}

	fn main() {
		let pair = Pair { first: "hello", second: "world" };
		println!("The pair is: ({}, {})", pair.first, pair.second);
	}
	```

- In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create structs that can work with different lifetimes.

## Lifetimes in Enums

- Lifetimes can be used in enum definitions to create enums that can work with different lifetimes.

	```rust
	enum List<'a> {
		Empty,
		Cons(&'a str, Box<List<'a>>),
	}

	fn main() {
		let list = List::Cons("hello", Box::new(List::Empty));
		println!("The list is: {:?}", list);
	}
	```

- In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create enums that can work with different lifetimes.

# Generic Types, Traits, and Lifetimes

## Generic Types, Traits, and Lifetimes in Functions

- Generic types, traits, and lifetimes can be used in function signatures to create functions that can work with different types, traits, and lifetimes.

	```rust
	fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T
	where
		T: std::cmp::PartialOrd,
	{
		if x.partial_cmp(y).unwrap() > std::cmp::Ordering::Equal {
			x
		} else {
			y
		}
	}

	struct Pair<'a, T> {
		first: &'a T,
		second: &'a T,
	}

	enum List<'a, T> {
		Empty,
		Cons(&'a T, Box<List<'a, T>>),
	}

	fn main() {
		let x = 5;
		let y = 10;
		let longest = longest(&x, &y);
		println!("The longest number is: {}", longest);
	}
	```

- In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create functions that can work with different types, traits, and lifetimes.

## Generic Types, Traits, and Lifetimes in Structs

- Generic types, traits, and lifetimes can be used in struct definitions to create structs that can work with different types, traits, and lifetimes.

	```rust
	struct Pair<'a, T> {
		first: &'a T,
		second: &'a T,
	}

	fn main() {
		let pair = Pair { first: 5, second: 10 };
		println!("The pair is: ({}, {})", pair.first, pair.second);
	}
	```

- In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create structs that can work with different types, traits, and lifetimes.

## Generic Types, Traits, and Lifetimes in Enums

- Generic types, traits, and lifetimes can be used in enum definitions to create enums that can work with different types, traits, and lifetimes.

	```rust
	enum List<'a, T> {
		Empty,
		Cons(T, Box<List<'a, T>>),
	}

	fn main() {
		let list = List::Cons(5, Box::new(List::Empty));
		println!("The list is: {:?}", list);
	}
	```

- In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create enums that can work with different types, traits, and lifetimes.

## Lifetime Elision Rules
- Rust has some lifetime elision rules that allow you to omit lifetimes in certain situations. These rules help reduce the need to write explicit lifetime annotations.

- The three main rules are:

	- Each parameter that is a reference gets its own lifetime parameter.
	- If there is exactly one input lifetime, it is assigned to all output lifetimes.
	- If there are multiple input lifetimes, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetimes.

### Example: Without Explicit Lifetimes
- For example, in the function signature below, Rust can infer the lifetimes based on these rules:
	```rust
	fn first_word(s: &str) -> &str {
		let bytes = s.as_bytes();
		for (i, &item) in bytes.iter().enumerate() {
			if item == b' ' {
				return &s[0..i];
			}
		}
		&s[..]
	}
	```
- Here, Rust automatically infers the lifetimes and knows that the return reference’s lifetime is tied to the input reference’s lifetime.


## Summary of Key Concepts
- Lifetimes ensure references are valid for as long as they are needed, preventing dangling references.
- Lifetime Annotations ('a) are used when the compiler cannot infer lifetimes automatically.
- Lifetime Elision Rules reduce the need to write lifetimes explicitly in simple cases.
- Structs with Lifetimes ensure that any references in a struct are valid as long as the struct itself.
- The 'static Lifetime denotes references that are valid for the entire duration of the program.
- Multiple Lifetimes can be used in functions to express complex relationships between different references.