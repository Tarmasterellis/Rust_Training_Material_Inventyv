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

In the above example, `Printable` is a trait that defines a `print` method. This method can be called on any type that implements the `Printable` trait.

## Trait Bounds

Trait bounds are used to specify the types that a generic type parameter must implement. They are defined using the `impl` keyword and can be used in function signatures, struct definitions, and enum definitions.

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

In the above example, `Printable` is a trait that defines a `print` method. This method can be called on any type that implements the `Printable` trait. The `print` method is defined for the `Pair` struct, which implements the `Printable` trait.