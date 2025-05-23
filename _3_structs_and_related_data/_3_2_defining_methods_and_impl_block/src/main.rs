/* Defining Methods and IMPL Block

	A method is a function that is defined inside a struct and can be called on instances of that struct.
	
	To define a method, use the `fn` keyword followed by the name of the method and a set of parentheses.
	
	Inside the parentheses, define the parameters that the method takes.
	
	The return type of the method is specified after an arrow (`->`).
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

impl Person {
	fn greet(&self, name: String) -> String {
		format!("Hello, {}!", name)
	}

	fn get_student_status(&self) -> bool {
		self.is_student
	}

	fn get_favorite_color(&self) -> String {
		self.favorite_color.clone()
	}

	fn get_phone_number(&self) -> String {
		self.phone_number.clone()
	}

	fn get_email(&self) -> String {
		self.email.clone()
	}

	fn get_address(&self) -> String {
		self.address.clone()
	}

	fn get_person_info(&self) -> String {
		format!("Name: {}, Age: {}, Height: {}, Weight: {}, Is Student: {}, Favorite Color: {}, Phone Number: {}, Email: {}, Address: {}",
			self.name, self.age, self.height, self.weight, self.is_student, self.favorite_color, self.phone_number, self.email, self.address)
	}

	fn get_person_info_with_args(&self, name: &str, age: u8, height: f32, weight: f32, is_student: bool, favorite_color: &str, phone_number: &str, email: &str, address: &str) -> String {
		format!("Name: {}, Age: {}, Height: {}, Weight: {}, Is Student: {}, Favorite Color: {}, Phone Number: {}, Email: {}, Address: {}",
			name, age, height, weight, is_student, favorite_color, phone_number, email, address)
	}
}

fn main() {
	let person = Person {
		name: String::from("Jhon"),
		age: 25,
		height: 1.75,
		weight: 70.0,
		is_student: true,
		favorite_color: String::from("blue"),
		phone_number: String::from("123-456-7890"),
		email: String::from("Jhon@example.com"),
		address: String::from("123 Main St, Anytown, USA"),
	};

	println!("{}", person.greet(String::from("Jhon")));
	println!("{}", person.get_student_status());
	println!("{}", person.get_favorite_color());
	println!("{}", person.get_phone_number());
	println!("{}", person.get_email());
	println!("{}", person.get_address());
	println!("{}", person.get_person_info());
	println!("{}", person.get_person_info_with_args(&person.name, person.age, person.height, person.weight, person.is_student, &person.favorite_color, &person.phone_number, &person.email, &person.address));

	let person2 = Person {
		name: String::from("Jhon"),
		age: 25,
		height: 1.75,
		weight: 70.0,
		is_student: true,
		favorite_color: String::from("blue"),
		phone_number: String::from("123-456-7890"),
		email: String::from("Jhon@example.com"),
		address: String::from("123 Main St, Anytown, USA"),
	};

	println!("{}", person2.greet(String::from("Jhon")));
	println!("{}", person2.get_student_status());
	println!("{}", person2.get_favorite_color());
	println!("{}", person2.get_phone_number());
	println!("{}", person2.get_email());
	println!("{}", person2.get_address());
	println!("{}", person2.get_person_info());
	println!("{}", person.get_person_info_with_args(&person.name, person.age, person.height, person.weight, person.is_student, &person.favorite_color, &person.phone_number, &person.email, &person.address));
}