/* Serializing with serde_json

	serde_json is a powerful and easy-to-use JSON serialization and deserialization library for Rust.

	To use serde_json, add it as a dependency in your Cargo.toml file:

	[dependencies]
	serde_json = "1.0"

	Then, import the serde_json crate in your code:

	use serde_json;

	Let's start by creating a struct called `Person` that has four fields: `name`, `age`, `height`, and `weight`.

	We will create an instance of the `Person` struct and serialize it to a JSON string using serde_json.
*/

// Importing serde_json and serde for serialization
use serde_json;
use serde::Serialize;

// Defining a struct with Serialize trait
#[derive(Serialize)]
// #[allow(dead_code)]
struct Person {
	name: String,
	age: u8,
	height: f32,
	weight: f32,
}

// #[allow(dead_code)]
fn main() {
	// Creating an instance of the Person struct
	let person = Person {
		name: String::from("Ellis"),
		age: 25,
		height: 1.75,
		weight: 70.0,
	};

	// Serializing the Person struct to a JSON string
	let json_string = serde_json::to_string(&person).unwrap();

	// Printing the JSON string
	println!("{}", json_string);
}