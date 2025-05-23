/* Deserializing with serde_json

	serde_json is a powerful and easy-to-use JSON serialization and deserialization library for Rust.

	To use serde_json, add it as a dependency in your Cargo.toml file:

	[dependencies]
	serde_json = "1.0"

	Then, import the serde_json crate in your code:

	use serde_json;

	Let's start by creating a struct called `Person` that has four fields: `name`, `age`, `height`, and `weight`.

	We will create a JSON string containing the serialized `Person` struct and deserialize it to a `Person` struct using serde_json.
*/

// Importing serde_json and serde for deserialization
use serde_json;
use serde::Deserialize;

// Defining a struct with Deserialize trait
#[derive(Deserialize)]
#[allow(dead_code)]
struct Person {
	name: String,
	age: u8,
	height: f32,
	weight: f32,
}

#[allow(dead_code)]
fn main() {
	// Creating a JSON string containing the serialized Person struct
	let json_string = r#"{"name": "Jhon", "age": 25, "height": 1.75, "weight": 70.0}"#;

	// Deserializing the JSON string to a Person struct
	let person: Person = serde_json::from_str(json_string).unwrap();

	// Printing the deserialized Person struct
	println!("Name: {}", person.name);
	println!("Age: {}", person.age);
	println!("Height: {}", person.height);
	println!("Weight: {}", person.weight);
}