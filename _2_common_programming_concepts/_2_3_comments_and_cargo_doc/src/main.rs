/* Comments and Cargo Doc
	Comments are used to add explanatory notes to the code. They are ignored by the compiler and do not affect the execution of the program.

	Cargo Doc is used to generate documentation for the code. It is used by the compiler to generate documentation for the code.

	Comments are single-line comments that start with two forward slashes (//).
	They are used to add explanatory notes to the code.
	Comments are ignored by the compiler and do not affect the execution of the program.

	Cargo Doc is multi-line comments that start with two forward slashes (/*) and end with two forward slashes (*/).
	They are used to generate documentation for the code.
*/

/// This is a documentation comment for a function
fn add(x: i32, y: i32) -> i32 {
	x + y
}

/// This is a documentation comment for a module Sosandfkjsdbgfhlbsdfm. v;ndf 
mod my_module {
	// This is a single-line comment

	/* This is a
	multi-line comment */

	/// This is a documentation comment for a function jahsgdhkjasb;dkbsad;hbnk
	pub fn add(x: i32, y: i32) -> i32 {
		x + y
	}
}

fn main() {
	// This is a single-line comment

	/* This is a
	   multi-line comment */

	// This is a single-line comment

	/* This is a
	   multi-line comment */

	add(5, 10); // The value of x is: 15

	my_module::add(5, 10); // The value of x is: 15

}