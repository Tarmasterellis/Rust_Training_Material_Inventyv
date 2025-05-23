/*
	Ownership, Borrowing and References
		Ownership
			Ownership is the concept of who owns a piece of memory.
			In Rust, every value has a variable that’s called its owner.
			There can only be one owner at a time.
			When the owner goes out of scope, the value will be dropped.

			Ownership can be transferred to another variable using the let keyword.
			This is called moving ownership.

			Ownership Rules:
				1. Each value in Rust has a variable that’s called its owner.
				2. There can only be one owner at a time.
				3. When the owner goes out of scope, the value will be dropped.
				4. Ownership can be transferred to another variable using the let keyword.
					- This is called moving ownership.
				5. References are a way to refer to a value without taking ownership of it.
					- References are immutable by default.
					- References can be mutable by using the mut keyword.
					- References can be dereferenced using the * operator.
					- References can be used to borrow values from a variable.
						- Borrowing allows you to use a value without taking ownership of it.
						- Borrowing is a way to share ownership of a value with another variable.
						- Borrowing can be done using references, slices, and arrays.
						- References can be either immutable or mutable.
							- Immutable references cannot be changed once they are created.
							- Mutable references can be changed after they are created.
		Borrowing
			Borrowing is a way to use a value without taking ownership of it.
			It allows you to use a value in multiple places without having to copy it.
			Borrowing is a way to share ownership of a value with another variable.
		References
			References are a way to refer to a value without taking ownership of it.
			They are immutable by default.
			References can be mutable by using the mut keyword.
			References can be dereferenced using the * operator.
			References can be used to borrow values from a variable.
			Borrowing allows you to use a value without taking ownership of it.
			Borrowing is a way to share ownership of a value with another variable.
			Borrowing can be done using references, slices, and arrays.
			References can be either immutable or mutable.
			Immutable references cannot be changed once they are created.
			Mutable references can be changed after they are created.
*/

fn main() {
	// let s1 = String::from("Hello");  // s1 owns the string "Hello"
	// let s2 = s1;  // Ownership is transferred to s2, s1 is no longer valid

	// // println!("{}", s1);  // This would cause a compile error: "value borrowed here after move"
	// println!("{}", s2);  // s2 is now the owner of the string
	
	// let s1 = String::from("Hello");
	// let s2 = s1.clone();  // Creates a deep copy of the string

	// println!("s1: {}, s2: {}", s1, s2);  // Both s1 and s2 are valid

	// let s = String::from("Hello");

	// let len = calculate_length(&s);  // Borrow s as immutable reference

	// println!("The length of '{}' is {}", s, len);  // s is still valid here

	// let mut s = String::from("Hello");

	// println!(change(&mut s));  // s is no longer valid here

	// println!("{}", s);  // s is still valid, but its value has been changed

	// let mut s = String::from("Hello");
	
	// let r1 = &s;  // Immutable borrow
	// let r2 = &s;  // Immutable borrow
	// let r3 = &mut s;  // Mutable borrow - causes an error
	
	// println!("{}, and {}", r1, r3);  // 
	// println!("{}, {}, and {}", r1, r2, r3);  // Error: cannot borrow `s` as mutable because it's already borrowed immutably

	let r;

	{
		let x = 5;
		r = &x;  // r tries to borrow x, but x will be dropped at the end of this inner scope
		// println!("{}", r);  // Error: r is invalid because x is dropped
	}

	println!("{}", r);  // Error: r is invalid because x is dropped

}

// fn calculate_length(s: &String) -> usize {
// 	s.len()  // We are borrowing s, not taking ownership
// }

// fn change(s: &mut String) {
// 	s.push_str(", world!");  // Mutate the borrowed value
// 	println!("{}", s);  // s is still valid here
// }