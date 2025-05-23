#[allow(dead_code)]
fn main() {
	// Creating a new empty vector using Vec::new
	let mut numbers: Vec<i32> = Vec::new();

	// Adding elements to the vector
	numbers.push(1);
	numbers.push(2);
	numbers.push(3);

	// Creating a vector using the vec! macro
	let mut other_numbers = vec![10, 20, 30, 40];

	// Accessing elements
	println!("First element: {}", numbers[0]);

	// Removing the last element
	other_numbers.pop();

	// Iterating over the vector
	for num in &numbers {
		println!("Number: {}", num);
	}

	let vector: Vec<i32> = Vec::new();  // Vector to hold integers
	println!("Vector: {:?}", vector);

	let vector_initial = vec![1, 2, 3, 4, 5];  // Vector with predefined integer values
	println!("Vector: {:?}", vector_initial);

	let vector_capacity: Vec<i32> = Vec::with_capacity(10);  // Pre-allocate space for 10 elements
	println!("Vector: {:?}", vector_capacity);

	// Accessing Vector Elements
	let vector = vec![1, 2, 3];
	let first = vector[0];  // Accessing element using index
	let second = vector.get(1);  // Using the `get` method (returns Option)

	println!("First element: {:?}", vector);
	println!("First element: {}", first);
	println!("Second element: {}", second.unwrap());

	// Different Types of Vectors
	let integers: Vec<i32> = vec![1, 2, 3, 4];
	println!("{:?}", integers);

	let strings: Vec<String> = vec![
		String::from("Hello"),
		String::from("World"),
	];
	println!("{:?}", strings);

	let tuples: Vec<(i32, char)> = vec![
		(1, 'a'),
		(2, 'b'),
	];
	println!("{:?}", tuples);

	#[derive(Debug)]
	struct Person {
		name: String,
		age: u32,
	}

	let people: Vec<Person> = vec![
		Person { name: String::from("Alice"), age: 30 },
		Person { name: String::from("Bob"), age: 25 },
	];
	println!("{:?}", people);

	#[derive(Debug)]
	enum Fruit {
		Apple,
		Banana,
		Orange,
	}
	let fruits: Vec<Fruit> = vec![Fruit::Apple, Fruit::Banana, Fruit::Orange];

	println!("{:?}", fruits);

	// Example: Vector of Structs
	struct Point {
		x: f64,
		y: f64,
	}

	let points = vec![
		Point { x: 1.0, y: 2.0 },
		Point { x: 3.0, y: 4.0 },
	];

	for point in &points {
		println!("Point: ({}, {})", point.x, point.y);
	}

	// Iterating Over Vectors
	let vector = vec![10, 20, 30, 40];

	// Iterating using a for loop
	for val in &vector {
		println!("{}", val);
	}

	// Using iterators
	vector.iter().for_each(|val| println!("{}", val));
}
