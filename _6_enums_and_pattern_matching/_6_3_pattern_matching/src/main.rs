/*
	Pattern Matching
		Pattern matching in Rust is a powerful feature that allows you to compare a value against a set of patterns and execute code based on which pattern matches.
		It's used primarily in match expressions, but also in if let, while let, and even function parameters.
		Patterns can be simple or complex, involving destructuring data types like enums, structs, tuples, and arrays.
*/


// Pattern matching allows you to destructure complex data types like structs.
struct Point {
	x: i32,
	y: i32,
}
		
fn main() {
	let number = 7;

	match number {
		1 => println!("One!"),
		2 => println!("Two!"),
		3 => println!("Three!"),
		4 | 5 | 6 => println!("Four, Five, or Six!"),  // Multiple patterns in one arm
		7..=10 => println!("Between Seven and Ten!"),  // Range pattern
		_ => println!("Something else!"),  // Catch-all pattern
	}

	// Pattern matching allows you to destructure complex data types like structs.
	let p = Point{x: 10, y: 20};

	match p {
		Point { x: 0, y } => println!("On the y-axis at {}", y), // The first arm matches points where x is 0 and binds y to its value.
		Point { x, y: 0 } => println!("On the x-axis at {}", x), // The second arm matches points where y is 0 and binds x to its value.
		Point { x, y } => println!("Point at ({}, {})", x, y), // The third arm matches any point and binds both x and y to variables.
	}


	// Pattern matching is particularly powerful when used with enums.
	let msg = Message::Quit;
	process_message(msg);
	let msg = Message::Move { x: 0, y: 0 };
	process_message(msg);
	let msg = Message::Write(String::from("Hello, world!"));
	process_message(msg);
	let msg = Message::ChangeColor(255, 0, 255);
	process_message(msg);

	// Pattern Matching with tuples
	let pair = (0, -2);


	match pair {
		(0, y) => println!("First is 0, second is {}", y),
		(x, 0) => println!("First is {}, second is 0", x),
		_ => println!("Neither is 0"),
	}

	// Patter Matching with Arrays
	let arr = [1, 2, 3];

	match arr {
		[1, 2, 3] => println!("Matched the array [1, 2, 3]!"),
		[1, _, 3] => println!("Matched with first 1 and last 3, but middle can be anything."),
		[1, ..] => println!("Starts with 1"),
		_ => println!("Didn't match"),
	}

	// You can add additional conditions to patterns with if guards
	let number = Some(7);

	match number {
		Some(x) if x % 2 == 0 => println!("Even number: {}", x),
		Some(x) => println!("Odd number: {}", x),
		None => println!("No number"),
	}

	
}

// Pattern matching is particularly powerful when used with enums.
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
	match msg {
		Message::Quit => println!("Quit message received."),
		Message::Move { x, y } => println!("Move to ({}, {}).", x, y),
		Message::Write(text) => println!("Text: {}", text),
		Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {}).", r, g, b),
	}
}