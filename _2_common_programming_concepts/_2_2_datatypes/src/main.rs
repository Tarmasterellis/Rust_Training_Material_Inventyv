/* Data Types

	Data Types
		1. Rust has several built-in data types, including integers, floating-point numbers, booleans, characters, strings, and arrays.
		2. Each data type has a specific range of values it can represent and a specific set of operations that can be performed on it.
		3. Rust has a powerful type inference system that can automatically determine the type of a variable based on its value.
		4. Rust provides several ways to convert between different data types.

	Datatypes are of two types:
		1. Primitive Datatypes: Datatypes that are built-in to the language.
		2. Compound Datatypes: Datatypes that are defined by the programmer.
	
	Primitive Datatypes
		1. Primitive datatypes are the most basic datatypes in Rust.
		2. They are the building blocks of the language and are used to represent the most basic values.
		3. Primitive datatypes include integers, floating-point numbers, booleans, characters, and strings.
		4. Primitive datatypes are immutable by default, cannot be changed once they are assigned a value.
		6. Primitive datatypes can be used in expressions and assigned to
			- variables
			- patterns
			- function
			- signatures
			- struct definitions
			- enum definitions
			- trait definitions
			- generics
			- trait bounds
			- type annotations.
	
	Compound Datatypes
		1. Compound datatypes are datatypes that are defined by the programmer.
		2. They are used to represent more complex values and can be used to create more complex data structures.
		3. Compound datatypes can be used in expressions and assigned to
			- variables.
			- patterns.
			- function signatures.
			- struct definitions.
			- enum definitions.
			- trait definitions.
			- generics.
			- trait bounds.
			- type annotations.

	Primitive Datatypes
		1. Integers
			- Signed Integers - can be Positive or Negative
				- i8
				- i16
				- i32
				- i64
				- i128
			- Unsigned Integers - can only be positive
				- u8
				- u16
				- u32
				- u64
				- u128
		2. Floating-point numbers - can be positive or negative
			- f32
			- f64
		3. Booleans - can only be one of the two types
			- true
			- false
		4. Characters - can only be one of the following types
			- char
				- Unicode Scalar Value
				- UTF-8
				- ASCII
				- UTF-16
				- UTF-32
				- C-style Strings
					- \0
					- \xNN
					- \u{NNNN}
					- \u{NNNNNNNN}
					- \U{NNNNNNNNNNNN}
					- \N{name}
				- Byte Strings
					- b"string"
					- br"string"
					- br##"string"##
					- b'c'
					- b'\xFF'
					- b'\u{FF}'
					- b'\u{FF00}'
					- b'\U{FF0000}'
					- b'\N{LATIN SMALL LETTER A}'
					- b"string"
					- br"string"
					- br##"string"##
	
	Compound Datatypes
		1. Arrays
			- Arrays are a collection of elements of the same type.
			- Arrays are fixed-size collections of elements of the same type.
			- Arrays are represented by a pointer to a contiguous sequence of elements.
			- Arrays can be created using the
				- "array literal" syntax.
			- Arrays can be indexed using the
				- index operator.
			- Arrays can be sliced using the
				- slice operator.
			- Arrays can be iterated over using the
				- for loop.
		2. Strings
			- Strings are a collection of characters.
			- Strings are fixed-size collections of characters.
			- Strings are represented by a pointer to a contiguous sequence of characters.
			- Strings can be created using the
				- "string literal" syntax.
			- Strings can be indexed using the
				- index operator.
			- Strings can be sliced using the
				- slice operator.
			- Strings can be iterated over using the
				- for loop.
		3. Tuples
			- Tuples are a collection of elements of different types.
			- Tuples are fixed-size collections of elements of different types.
			- Tuples are represented by a tuple struct.
			- Tuples can be created using the
				- "tuple literal" syntax.
			- Tuples can be indexed using the
				- index operator.
			- Tuples can be iterated over using the
				- for loop.
		4. String Slices
			- String slices are a collection of characters.
			- String slices are fixed-size collections of characters.
			- String slices are represented by a pointer to a contiguous sequence of characters.
			- String slices can be created using the
				- "string literal" syntax.
			- String slices can be indexed using the
				- index operator.
			- String slices can be sliced using the
				- slice operator.
			- String slices can be iterated over using the
				- for loop.
				
*/

fn main() {
	// // integers
	// let x: i32 = 5;
	// println!("The value of x is: {x}");
	
	// let mut y: i32 = 5;
	// println!("The value of y is: {y}");
	// y = 10;
	// println!("The value of y is: {y}");

	// let z: i32 = 5;
	// println!("The value of z is: {z}");

	// // Floating-point numbers
	// let y: f64 = 3.14;
	// println!("The value of y is: {y}");

	// let mut z: f64 = 3.14;
	// println!("The value of z is: {z}");
	// z = 10.0;
	// println!("The value of z is: {z}");

	// // booleans
	// let z: bool = true;
	// println!("The value of z is: {z}");

	// // characters
	// let a: char = 'a';
	// println!("The value of a is: {a}");

	// strings
	let mut b: &str = "hello";
	println!("The value of b is: {b}");
	
	b = "Something Else";
	println!("The value of b is: {b}");

	let mut c: String = "hello".to_string();
	println!("The value of c is: {c}");
	
	c = "This is a string...!".to_string();
	println!("The value of c is: {c}");

	let d = "hello".to_string();
	println!("The value of d is: {d}");

	let e = String::from("hello");
	println!("The value of e is: {e}");

	// // arrays
	// let c: [i32; 5] = [1, 2, 3, 4, 5];
	// println!("The value of c is: {:?}", c);

	// let d: [i32; 5] = [1, 2, 3, 4, 5];	
	// println!("The value of d is: {:?}", d);
	// let mut new_array = [0; 6];
	// new_array[..5].copy_from_slice(&d);
	// new_array[5] = 6;
	// println!("The value of new_array is: {:?}", new_array);

	// // tuples
	// let d: (i32, f64, bool) = (1, 3.14, true);
	// println!("The value of d is: {:?}", d);

	// let mut e: (i32, f64, bool) = (1, 3.14, true);
	// println!("The value of e is: {:?}", e);
	// e.0 = 10;
	// println!("The value of e is: {:?}", e);

	// // String Slices
	// let e: &str = "hello";
	// println!("The value of e is: {e}");

	// let mut f: &str = "hello";
	// println!("The value of f is: {f}");
	// f = "world";
	// println!("The value of f is: {f}");

}