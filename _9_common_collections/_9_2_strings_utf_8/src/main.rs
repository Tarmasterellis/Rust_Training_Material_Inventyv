/* Strings - UTF-8 */

fn main() {
	// UTF-8 Basics
	let hello = String::from("Hello, 世界!"); // Unicode characters (世界) and ASCII
	println!("{}", hello);

	let emoji = "😊 Rust".to_string();
	println!("{}", emoji);

	let mut s = String::new(); // Empty string
	s.push_str("Hello, 世界!");
	println!("{}", s);

	// UTF-8 Encoding and Byte Representation
	let s = String::from("Hola! 😊");
	for b in s.as_bytes() {
		print!("{} ", b); // Prints the UTF-8 bytes
	}

	// Iterating Over Characters
	let s = String::from("Hola! 😊");
	for c in s.chars() {
		println!("{}", c);
	}

	// Iterating Over Bytes
	let s = String::from("Hola! 😊");
	for b in s.bytes() {
		println!("{}", b);
	}

	// String Slicing and UTF-8 Safety
	let s = String::from("Hola! 😊");
	
	// This works because it's slicing at character boundaries
	let slice = &s[0..5]; // "Hola!"
	println!("{}", slice);
	
	// This would cause a panic if you try to slice in the middle of a multi-byte character
	let invalid_slice = &s[0..7]; // Panics at runtime
    println!("{:?}", invalid_slice);

	// Converting Between Strings and Bytes
	let s = String::from("Hello!");
	let bytes = s.into_bytes(); // Converts into a Vec<u8>
	println!("{:?}", bytes); // [72, 101, 108, 108, 111, 33]

	let bytes = vec![72, 101, 108, 108, 111, 33];
	let s = String::from_utf8(bytes).unwrap(); // Converts bytes to a String
	println!("{}", s); // "Hello!"
}