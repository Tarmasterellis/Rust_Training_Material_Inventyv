# Common Programming Concepts

## Variables

Variables are used to store data that can be used later in the program. They are declared using the `let` keyword followed by the variable name and an optional type annotation. The variable name can only contain letters, numbers, and underscores, and cannot start with a number.

```rust
fn main() {
	let mut x = 5;
	println!("The value of x is: {x}");
	x = 6;
	println!("The value of x is: {x}");
}
```

Variables can be reassigned to a new value using the `let` keyword followed by the variable name and the new value.

```rust
let x = 5;
x = 10;
```

## Variable Scope

Variables have a scope that determines where they can be accessed in the program. The scope of a variable is the part of the program where the variable is valid and can be used. There are three types of variable scope in Rust:

- Local: Variables declared inside a function are local to that function and can only be accessed within that function.

- Block: Variables declared inside a block are local to that block and can only be accessed within that block.

- Global: Variables declared outside of any function or block are global and can be accessed from anywhere in the program.

```rust
fn main() {
	let x = 5;

	{
		let y = 10;
	}

	println!("x: {}", x);
	println!("y: {}", y);
}
```

In this example, the variable `x` is local to the `main` function and can only be accessed within that function. The variable `y` is local to the block and can only be accessed within that block.

## Mutability

Variables can be either mutable or immutable. Mutable variables can be changed after they are assigned a value, while immutable variables cannot be changed.

```rust
let x = 5; // x is immutable
let mut y = 10; // y is mutable

y = 20; // This is allowed because y is mutable
x = 20; // This is not allowed because x is immutable
```

Mutable variables can be changed to a new value using the `let` keyword followed by the variable name and the new value.

```rust
let mut x = 5;
x = 10;
```

Immutable variables cannot be changed to a new value using the `let` keyword. Instead, the value must be reassigned to a new value.

```rust
let x = 5;
x = 10; // This is not allowed because x is immutable
```

## Data Types

Rust has several built-in data types, including integers, floating-point numbers, booleans, characters, strings, and arrays. Each data type has a specific range of values it can represent and a specific set of operations that can be performed on it.

```rust
let x: i32 = 5; // i32 is a 32-bit signed integer
let y: f64 = 3.14; // f64 is a 64-bit floating-point number
let z: bool = true; // bool is a boolean
let a: char = 'a'; // char is a Unicode scalar value
let b: &str = "hello"; // &str is a string slice
let c: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 i32s
```

## Type Inference

Rust has a powerful type inference system that can automatically determine the type of a variable based on its value. This means that you don't always have to explicitly specify the type of a variable.

```rust
fn main() {
	let x = 5; // x is inferred to be an i32
	let y = 3.14; // y is inferred to be a f64
	let z = true; // z is inferred to be a bool
	let a = 'a'; // a is inferred to be a char
	let b = "hello"; // b is inferred to be a &str
	let c = [1, 2, 3, 4, 5]; // c is inferred to be an array of i32s
}
```

## Type Conversion

Rust provides several ways to convert between different data types. This includes implicit conversions, explicit conversions, and type casting.

Implicit conversions are performed automatically by the compiler when it is able to infer the type of a value. For example, the value `5` can be implicitly converted to an `i32` because it fits within the range of an `i32`.

```rust
let x: i32 = 5; // x is an i32
let y: f64 = 3.14; // y is a f64

let sum = x + y; // sum is an i32
```

Explicit conversions are performed using the `as` keyword followed by the target type. For example, the value `5` can be explicitly converted to an `i32` using the `as` keyword.

```rust
let x: i32 = 5; // x is an i32
let y: f64 = 3.14; // y is a f64

let sum = x as i32 + y as i32; // sum is an i32
```

Type casting is performed using the `as` keyword followed by the target type. For example, the value `5` can be cast to an `i32` using the `as` keyword.

```rust
let x: i32 = 5; // x is an i32
let y: f64 = 3.14; // y is a f64

let sum = x as i32 + y as i32; // sum is an i32
```

## Shadowing

Variables can be shadowed by other variables with the same name. This can lead to unexpected behavior and bugs in the program.

```rust
fn main() {
	let x = 5;
	let x = 10; // This is allowed because x is shadowed
	println!("The value of x is: {x}");
}
```

In this example, the variable `x` is shadowed by the variable `x` declared inside the `main` function. This can lead to unexpected behavior and bugs in the program.

## Variable Lifetime

Variables have a lifetime that determines how long they can be used in the program. The lifetime of a variable is the part of the program where the variable is valid and can be used. There are two types of variable lifetime in Rust:

- Static: Variables declared outside of any function or block have a static lifetime and can be accessed from anywhere in the program.

- Dynamic: Variables declared inside a function or block have a dynamic lifetime and can only be accessed within that function or block.

```rust
fn main() {
	let x = 5;

	{
		let y = 10;
	}

	println!("x: {}", x);
	println!("y: {}", y);
}
```

In this example, the variable `x` has a static lifetime and can be accessed from anywhere in the program. The variable `y` has a dynamic lifetime and can only be accessed within the block.

## Comments and Documentation

Comments are used to add explanatory notes to the code. They are ignored by the compiler and do not affect the execution of the program.

```rust
// This is a single-line comment

/* This is a
   multi-line comment */
```

Documentation comments are used to add explanatory notes to the code. They are used by the compiler to generate documentation for the code.

```rust
/// This is a documentation comment for a function
fn add(x: i32, y: i32) -> i32 {
	x + y
}
```

Documentation comments can also be used to generate documentation for modules, structs, enums, traits, and more.

```rust
/// This is a documentation comment for a module
mod my_module {
	/// This is a documentation comment for a function
	fn add(x: i32, y: i32) -> i32 {
		x + y
	}
}
```

Documentation comments can also be used to generate documentation for items within a module, struct, enum, trait, or more.

```rust
/// This is a documentation comment for a struct
struct MyStruct {
	/// This is a documentation comment for a field
	field: i32,
}

/// This is a documentation comment for an enum
enum MyEnum {
	/// This is a documentation comment for a variant
	Variant,
}

/// This is a documentation comment for a trait
trait MyTrait {
	/// This is a documentation comment for a method
	fn my_method(&self);
}
```

Documentation comments can also be used to generate documentation for attributes.

```rust
#[derive(Debug)]
struct MyStruct {
	#[allow(dead_code)]
	field: i32,
}
```

## Control Flow

Rust provides several control flow statements, including `if` statements, `for` loops, `while` loops, and `match` expressions. These statements allow you to control the flow of execution in your program based on certain conditions.

```rust
fn main() {
	let x = 5;

	if x > 0 {
		println!("x is positive");
	} else if x < 0 {
		println!("x is negative");
	} else {
		println!("x is zero");
	}

	for i in 1..=5 {
		println!("i: {}", i);
	}

	while x > 0 {
		x -= 1;
	}

	match x {
		0 => println!("x is zero"),
		_ => println!("x is not zero"),
	}
}
```

In this example, the `if` statement checks if `x` is greater than 0. If it is, the code inside the block is executed. If not, the code inside the `else if` block is executed. If none of the conditions are met, the code inside the `else` block is executed.

The `for` loop iterates over a range of values. In this example, the loop iterates over the range `1..=5` and prints each value.

The `while` loop continues to execute as long as a certain condition is true. In this example, the loop continues to execute as long as `x` is greater than 0.

The `match` expression matches a value against a series of patterns. In this example, the value of `x` is matched against the patterns `0` and `_`. If the value matches the first pattern, the code inside the block is executed. If the value matches the second pattern, the code inside the block is executed.