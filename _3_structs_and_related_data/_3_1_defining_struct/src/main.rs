/* Defining Structs
	A struct is a custom data type that allows you to group together related data into a single unit.
	It is similar to a class in object-oriented programming languages.
	To define a struct, use the `struct` keyword followed by the name of the struct and a set of curly braces.
	Inside the curly braces, define the fields of the struct. Each field has a name and a type.
	The fields of a struct are accessed using dot notation. For example, to access the `name` field of the `person` struct,
	you would use `person.name`.
	You can also access the fields of a struct using the `struct_name.field_name` syntax.
	Let's define a struct called `Person` that has three fields: `name`, `age`, and `height`.
	We will create an instance of the `Person` struct and access its fields using dot notation.
*/

struct Person {
	name: String,
	age: u8,
	height: f32,
	weight: f32,
	is_student: bool,
	favorite_color: String,
	phone_number: String,
	email: String,
	address: String,
}

fn main() {
	// Creating an instance of the Person struct
	let person = Person {
		name: String::from("Ellis"),
		age: 25,
		height: 1.75,
		weight: 70.0,
		is_student: true,
		favorite_color: String::from("blue"),
		phone_number: String::from("123-456-7890"),
		email: String::from("ellis@example.com"),
		address: String::from("123 Main St, Anytown, USA"),
	};

	// Accessing the fields of the Person struct
	println!("Name: {}", person.name);
	println!("Age: {}", person.age);
	println!("Height: {}", person.height);
	println!("Weight: {}", person.weight);
	println!("Is Student: {}", person.is_student);
	println!("Favorite Color: {}", person.favorite_color);
	println!("Phone Number: {}", person.phone_number);
	println!("Email: {}", person.email);
	println!("Address: {}", person.address);
}