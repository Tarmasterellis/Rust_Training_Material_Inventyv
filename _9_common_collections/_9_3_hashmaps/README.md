# Hashmaps

## Hashmaps

	- A hashmap is a data structure that maps keys to values. It is similar to a dictionary in Python or a hash table in C++. Hashmaps are useful when you want to store data in a way that allows for fast lookups.

### Creating a Hashmap

	- To create a hashmap, you can use the `HashMap::new()` method. This method takes no arguments and returns a new, empty hashmap.

	```rust
	use std::collections::HashMap;

	let mut scores = HashMap::new();
	```

### Adding to a Hashmap

	- To add a key-value pair to a hashmap, you can use the `insert()` method. This method takes two arguments: the key and the value. The key can be any type that is `Hashable`, which means that it can be used as a key in a hashmap.

	```rust
	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Alice"), 10);
	scores.insert(String::from("Bob"), 20);
	scores.insert(String::from("Charlie"), 30);
	```

### Accessing Values in a Hashmap

	- To access the value associated with a key in a hashmap, you can use the `get()` method. This method takes a key as an argument and returns a `Option<&V>` where `V` is the type of the value associated with the key.

	```rust
	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Alice"), 10);
	scores.insert(String::from("Bob"), 20);
	scores.insert(String::from("Charlie"), 30);

	let alice_score = scores.get("Alice");
	let bob_score = scores.get("Bob");
	let charlie_score = scores.get("Charlie");
	```

### Updating Values in a Hashmap

	- To update the value associated with a key in a hashmap, you can use the `insert()` method. This method takes two arguments: the key and the value. If the key already exists in the hashmap, its value will be updated. If the key doesn't exist in the hashmap, a new key-value pair will be added.

	```rust
	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Alice"), 10);
	scores.insert(String::from("Bob"), 20);
	scores.insert(String::from("Charlie"), 30);

	scores.insert(String::from("Alice"), 50);

	let alice_score = scores.get("Alice");
	```

### Removing Values from a Hashmap

	- To remove a key-value pair from a hashmap, you can use the `remove()` method. This method takes a key as an argument and returns a `Option<V>` where `V` is the type of the value associated with the key.

	```rust
	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Alice"), 10);
	scores.insert(String::from("Bob"), 20);
	scores.insert(String::from("Charlie"), 30);

	scores.remove("Alice");

	let alice_score = scores.get("Alice");
	```

### Iterating Over a Hashmap

	- To iterate over the key-value pairs in a hashmap, you can use a `for` loop. The loop will iterate over the keys and values in the hashmap.

	```rust
	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Alice"), 10);
	scores.insert(String::from("Bob"), 20);
	scores.insert(String::from("Charlie"), 30);

	for (name, score) in &scores { // The & is required to borrow the hashmap
		println!("{}: {}", name, score);
	}
	```

### Hashmaps vs. HashSets

- Hashmaps and hashsets are both data structures that store key-value pairs. However, they have some important differences:

	- Hashmaps are more flexible than hashsets. A hashset is a collection of unique values, while a hashmap can store multiple values for the same key.
	- Hashmaps are generally slower than hashsets for lookups. This is because hashmaps need to perform a hash function on the key to find the correct bucket, while hashsets can use a binary search to find the correct value.
	- Hashmaps are generally more memory-efficient than hashsets. This is because hashmaps only store the key-value pairs that are currently in use, while hashsets need to store all of the values that have ever been added to the set.

- In general, if you need to store a collection of unique values, a hashset is a good choice. If you need to store a collection of values that can have multiple values for the same key, a hashmap is a good choice.

## Hashmaps in Practice

- Hashmaps are a powerful data structure that can be used in a variety of situations. Here are some examples of how you might use a hashmap in practice:

	- Storing scores for different players in a game.
	- Storing the contents of a web form.
	- Storing the state of a game.
	- Storing the configuration of a program.

- In each of these examples, the hashmap is used to store key-value pairs that represent different aspects of the data. The keys are typically strings or integers, and the values can be any type that is `Hashable`.

## Summary

- In this section, we learned about hashmaps and how to use them in Rust.
- Hashmaps are a powerful data structure that can be used to store key-value pairs in a way that allows for fast lookups.
- We also learned about the `Hashable` trait and how to implement it for our own types.
- Finally, we explored some common use cases for hashmaps and how they can be used in practice.