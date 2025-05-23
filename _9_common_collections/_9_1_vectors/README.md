# Vectors
- In Rust, a vector is a dynamic array that can grow and shrink in size. It is implemented in the standard library as the Vec<T> type, where T is the type of the elements stored in the vector. Vectors are heap-allocated, meaning they are stored on the heap and their size can be dynamically adjusted during runtime.

- Key Features of Vectors:
	- They can grow or shrink in size.
	- They provide indexed access to elements.
	- Vectors are stored in contiguous memory locations.
	- Elements are added or removed using methods like push and pop.

## Basic Syntax:
- To create a vector in rust, you can use either  the `vec::new()` function or the `vec![]` macro.

- Creating and Manupulating Vectors:
	```rust
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
	}
	```

### Different Ways to Implement Vectors:

- **Empty Vector with Type Annotation**: You can create an empty vector and specify the type of elements it will hold.

	```rust
	let mut vector: Vec<i32> = Vec::new();  // Vector to hold integers
	```
	
- **Vector with Initial Values**: You can initialize a vector with predefined values using the `vec![]` macro.

	```rust
	let vector = vec![1, 2, 3, 4, 5];  // Vector with predefined integer values
	```

- **Vector Using** ``Vec::with_capacity()``: You can create a vector with a specified initial capacity. This avoids reallocations when pushing elements, which can be more efficient if you know the number of elements in advance.

	```rust
	let mut vector: Vec<i32> = Vec::with_capacity(10);  // Pre-allocate space for 10 elements
	```

- **Accessing Vector Elements**: You can access vector elements by indexing or using the `get` method.

	```rust
	let vector = vec![1, 2, 3];
	let first = vector[0];  // Accessing element using index
	let second = vector.get(1);  // Using the `get` method (returns Option)
	```

### Common Operations on Vectors:

- **Adding Elements**: You can add elements to a vector using the `push()` method.
	```rust
	let mut vec = Vec::new();
	vec.push(5);  // Adds 5 to the vector
	```

- **Removing Elements**: Use `pop()` to remove the last element from the vector.
	```rust
	let mut vec = vec![1, 2, 3];
	vec.pop();  // Removes 3 from the vector
	```

- **Modifying Elements**: You can modify vector elements by accessing them via mutable references.
	```rust
	let mut vec = vec![10, 20, 30];
	vec[1] = 25;  // Modify the second element to 25
	```

### Different Types of Vectors:
- **Vector of Integers**: A vector that holds integers.
	```rust
	let integers: Vec<i32> = vec![1, 2, 3, 4];
	```

- **Vector of Strings**: A vector that holds String values.
	```rust
	let strings: Vec<String> = vec![
		String::from("Hello"),
		String::from("World"),
	];
	```

- **Vector of Tuples**: A vector that holds tuples.
	```rust
	let tuples: Vec<(i32, char)> = vec![
		(1, 'a'),
		(2, 'b'),
	];
	```

- **Vector of Structs**: You can have a vector that holds structs.
	```rust
	struct Person {
		name: String,
		age: u32,
	}

	let people: Vec<Person> = vec![
		Person { name: String::from("Alice"), age: 30 },
		Person { name: String::from("Bob"), age: 25 },
	];
	```
- **Vector of Enums**: A vector can hold enum values.
	```rust
	enum Fruit {
		Apple,
		Banana,
		Orange,
	}

	let fruits: Vec<Fruit> = vec![Fruit::Apple, Fruit::Banana];
	```

- **Example: Vector of Structs**
	```rust
	struct Point {
		x: f64,
		y: f64,
	}

	fn main() {
		let points = vec![
			Point { x: 1.0, y: 2.0 },
			Point { x: 3.0, y: 4.0 },
		];

		for point in &points {
			println!("Point: ({}, {})", point.x, point.y);
		}
	}
	```
- **Iterating Over Vectors**: You can iterate over vectors using a `for` loop or iterators.

	```rust
	let vector = vec![10, 20, 30, 40];

	// Iterating using a for loop
	for val in &vector {
		println!("{}", val);
	}

	// Using iterators
	vector.iter().for_each(|val| println!("{}", val));
	```