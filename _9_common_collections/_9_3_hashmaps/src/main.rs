/*
	Hashmaps
		- In Rust, a HashMap is a collection type that stores key-value pairs, allowing for fast lookups based on keys.
		- It is part of the standard library and is implemented in the std::collections module.
		- The keys in a HashMap must be unique, and they are hashed to determine their position in the map, making lookups efficient.

	Key Concepts
		- Hashing: Rust uses a hashing algorithm (typically SipHash) to convert keys into hash values. This allows for quick access to the associated values.
		- Ownership and Borrowing: As with all Rust collections, HashMap enforces ownership rules. Keys and values are typically stored by value, but you can also use references.
		- Generics: HashMap is a generic type, allowing you to specify the types of keys and values.

	Common Operations
		- Insertion: Use the insert method to add or update entries.
		- Access: Use the get method to retrieve values.
		- Removal: Use the remove method to delete entries.




	Hashset
		- A HashSet in Rust is a collection type that stores unique values, without any associated keys.
		- It is similar to a HashMap in terms of underlying implementation (using hashing for fast lookups) but focuses solely on the values themselves.
		- HashSet is part of the standard library, located in the std::collections module.

	Key Concepts
		- Uniqueness: Each value in a HashSet must be unique; attempting to insert a duplicate value will not result in an error, but the existing value will remain unchanged.
		- Ownership and Borrowing: Like other Rust collections, HashSet enforces ownership rules. Values are stored by value, but references can also be used.
		- Generics: HashSet is a generic type, allowing you to specify the type of the values it will hold.

	Common Operations
		- Insertion: Use the insert method to add values.
		- Checking for Existence: Use the contains method to check if a value is present.
		- Removal: Use the remove method to delete values.
		- Iteration: You can iterate over the values using a for loop.



	Difference Between Hashmap vs Hashset
	________________________________________________________________________________________________________________________________
	| Feature				| HashMap													| HashSet									|
	________________________________________________________________________________________________________________________________
	| Purpose				| Stores key-value pairs									| Stores unique values						|
	| Key-Value 			| Structure	Requires both keys and values					| Requires only values						|
	| Uniqueness			| Keys must be unique; values can repeat					| All values must be unique					|
	| Access				| Values are accessed via their keys						| Values are accessed directly				|
	| Insertions			| insert(key, value)										| insert(value)								|
	| Memory Overhead		| Typically has more overhead due to key storage			| Less overhead as only values are stored	|
	_________________________________________________________________________________________________________________________________

*/

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
	// hashmaps();
	hashsets();
}


// Custom Types as Keys
#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}



fn hashmaps() {
	// Basic Hashmap
	// Create a new HashMap
	let mut scores = HashMap::new();

	// Insert key-value pairs
	scores.insert(String::from("Alice"), 10);
	scores.insert(String::from("Bob"), 20);
	
	// Access a value by its key
	let alice_score = scores.get("Alice");
	match alice_score {
		Some(&score) => println!("Alice's score: {}", score),
		None => println!("Alice not found!"),
	}

	// Iterate over key-value pairs
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}


	// Default Types
	let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    println!("{:?}", map);



	// References
	let mut map = HashMap::new();
    let key1 = String::from("Key1");
    let value1 = 100;

    map.insert(&key1, value1);
    
    // Accessing a value using a reference
    if let Some(&value) = map.get(&key1) {
        println!("Value for {}: {}", key1, value);
    }




	// Iterators
	let keys = vec!["a", "b", "c"];
    let values = vec![1, 2, 3];

    let map: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }




	// Custom Types as Keys
	let mut map = HashMap::new();
    
    map.insert(Point { x: 0, y: 0 }, "Origin");
    map.insert(Point { x: 1, y: 1 }, "Point 1");
    
    for (point, name) in &map {
        println!("{:?}: {}", point, name);
    }
}



fn hashsets() {

	// Create a new HashSet
    let mut numbers = HashSet::new();

    // Insert values
    numbers.insert(1);
    numbers.insert(2);
    numbers.insert(3);
    
    // Attempting to insert a duplicate value
    let was_inserted = numbers.insert(2); // Returns false, as 2 is already present
    println!("Was 2 inserted again? {}", was_inserted);

    // Check if a value exists
    if numbers.contains(&3) {
        println!("The set contains 3");
    }

    // Iterate over the HashSet
    for number in &numbers {
        println!("{}", number);
    }



	// Default Types
	let mut set: HashSet<&str> = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("orange");

    println!("{:?}", set);




	// References
	let mut set = HashSet::new();
    let fruit1 = String::from("mango");
    let fruit2 = String::from("grape");

    set.insert(&fruit1);
    set.insert(&fruit2);
    
    // Checking existence
    if set.contains(&fruit1) {
        println!("Set contains mango");
    }




	// Iterators
	let values = vec![1, 2, 3, 4, 4, 5, 5 , 6, 7, 6, 7]; // Duplicate 4
    let set: HashSet<_> = values.into_iter().collect(); // Duplicates will be removed

    for value in &set {
        println!("{}", value);
    }




	// Custom Types
	let mut set = HashSet::new();
    
    set.insert(Point { x: 1, y: 2 });
    set.insert(Point { x: 1, y: 2 }); // Duplicate, will not be added

    for point in &set {
        println!("{:?}", point);
    }
}