# Generic Types

## Generic Types

- Generic types are types that can be parameterized with other types. They are defined using angle brackets (`<>`) and can be used in function signatures, struct definitions, and enum definitions.

	```rust
	fn print<T>(value: T) {
		println!("The value is: {}", value);
	}

	struct Pair<T> {
		first: T,
		second: T,
	}

	enum List<T> {
		Empty,
		Cons(T, Box<List<T>>),
	}
	```

- In the above example, `T` is a generic type that can be replaced with any type. This allows us to create functions, structs, and enums that can work with different types.

## Generic Functions

- Generic functions are functions that can be parameterized with other types. They are defined using angle brackets (`<>`) and can be used in function signatures, struct definitions, and enum definitions.

	```rust
	fn print<T>(value: T) {
		println!("The value is: {}", value);
	}

	struct Pair<T> {
		first: T,
		second: T,
	}

	enum List<T> {
		Empty,
		Cons(T, Box<List<T>>),
	}
	```

- In the above example, `T` is a generic type that can be replaced with any type. This allows us to create functions, structs, and enums that can work with different types.

## Generic Structs

- Generic structs are structs that can be parameterized with other types. They are defined using angle brackets (`<>`) and can be used in function signatures, struct definitions, and enum definitions.

	```rust
	fn print<T>(value: T) {
		println!("The value is: {}", value);
	}

	struct Pair<T> {
		first: T,
		second: T,
	}

	enum List<T> {
		Empty,
		Cons(T, Box<List<T>>),
	}
	```

- In the above example, `T` is a generic type that can be replaced with any type. This allows us to create functions, structs, and enums that can work with different types.

## Generic Enums

- Generic enums are enums that can be parameterized with other types. They are defined using angle brackets (`<>`) and can be used in function signatures, struct definitions, and enum definitions.

	```rust
	fn print<T>(value: T) {
		println!("The value is: {}", value);
	}

	struct Pair<T> {
		first: T,
		second: T,
	}

	enum List<T> {
		Empty,
		Cons(T, Box<List<T>>),
	}
	```

- In the above example, `T` is a generic type that can be replaced with any type. This allows us to create functions, structs, and enums that can work with different types.

## Generic Types in Functions

- Generic types can be used in function signatures to create functions that can work with different types.

	```rust
	fn print<T>(value: T) {
		println!("The value is: {}", value);
	}

	fn main() {
		let x = 5;
		print(x);
	}
	```

- In the above example, `T` is a generic type that can be replaced with any type. This allows us to create functions that can work with different types.

## Generic Types in Structs

- Generic types can be used in struct definitions to create structs that can work with different types.

	```rust
	struct Pair<T> {
		first: T,
		second: T,
	}

	fn main() {
		let pair = Pair { first: 5, second: 10 };
		println!("The pair is: ({}, {})", pair.first, pair.second);
	}
	```

- In the above example, `T` is a generic type that can be replaced with any type. This allows us to create structs that can work with different types.

## Generic Types in Enums

- Generic types can be used in enum definitions to create enums that can work with different types.

	```rust
	enum List<T> {
		Empty,
		Cons(T, Box<List<T>>),
	}

	fn main() {
		let list = List::Cons(5, Box::new(List::Empty));
		println!("The list is: {:?}", list);
	}
	```

- In the above example, `T` is a generic type that can be replaced with any type. This allows us to create enums that can work with different types.

# Traits

## Traits

- Traits in Rust are a way to define shared behavior between different types. A trait can be thought of as a collection of methods defined for a particular type. They allow you to define functionality that a type must provide in order to be considered an implementation of the trait. This is Rust's version of interfaces (in languages like Java or C#) or type classes (in Haskell).

- Rules for Traits
	- Defining a Trait: A trait defines a set of methods that a type must implement. Some methods can have default implementations, while others must be implemented by the type.

	- Implementing a Trait: Types implement traits using the impl keyword followed by the trait and the type. Once a type implements a trait, it can use the methods defined by the trait.

	- Trait Bounds: Traits can be used as constraints for generic types, allowing you to specify that a type must implement a certain trait for it to be used in a function or struct.

	- Multiple Traits: A type can implement multiple traits, allowing it to use multiple behaviors.

	- Trait Objects: Traits can be used as types, allowing for dynamic dispatch (runtime polymorphism) when you don’t know the exact type at compile time.

	- Orphan Rule: In Rust, you can only implement a trait for a type if either the trait or the type is defined in the current crate (module). This prevents conflicts when multiple external crates are involved.


- Traits are a way to define shared behavior across different types. They allow us to define methods that can be called on any type that implements the trait.

	```rust
	trait Printable {
		fn print(&self);
	}

	struct Pair {
		first: i32,
		second: i32,
	}

	impl Printable for Pair {
		fn print(&self) {
			println!("The pair is: ({}, {})", self.first, self.second);
		}
	}

	enum List {
		Empty,
		Cons(i32, Box<List>),
	}

	impl Printable for List {
		fn print(&self) {
			match self {
				List::Empty => println!("The list is empty"),
				List::Cons(value, tail) => {
					println!("The list is: {:?}", value);
					tail.print();
				}
			}
		}
	}
	```

- In the above example, `Printable` is a trait that defines a `print` method. This method can be called on any type that implements the `Printable` trait.

## Trait Bounds

- Trait bounds are used to specify the types that a generic type parameter must implement. They are defined using the `impl` keyword and can be used in function signatures, struct definitions, and enum definitions.

	```rust
	trait Printable {
		fn print(&self);
	}

	struct Pair {
		first: i32,
		second: i32,
	}

	impl Printable for Pair {
		fn print(&self) {
			println!("The pair is: ({}, {})", self.first, self.second);
		}
	}

	fn print<T: Printable>(value: T) {
		value.print();
	}

	fn main() {
		let pair = Pair { first: 1, second: 2 };
		print(pair);
	}
	```

- In the above example, `Printable` is a trait that defines a `print` method. This method can be called on any type that implements the `Printable` trait. The `print` method is defined for the `Pair` struct, which implements the `Printable` trait.

# Lifetimes

## Lifetimes

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

In the above example, `'a` is a lifetime that can be replaced with any lifetime. This allows us to create enums that can work with different lifetimes.

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

# Conclusion

- In this section, we learned about generic types, traits, and lifetimes in Rust. Generic types allow us to create functions, structs, and enums that can work with different types. Traits allow us to define shared behavior across different types. Lifetimes allow us to specify the lifetime of a reference. We also learned how to use generic types, traits, and lifetimes in function signatures, struct definitions, and enum definitions.