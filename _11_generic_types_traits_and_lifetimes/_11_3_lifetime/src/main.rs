/*
	Lifetimes

	- Rules of Lifetimes:
		- Every reference has a lifetime: The compiler tracks the scope in which references are valid and ensures they don't outlive the data they point to.
		- Lifetimes are inferred: In many cases, Rust is able to infer lifetimes automatically, especially for simple cases. But sometimes lifetimes must be explicitly specified.
		- Lifetime annotations don’t change how long a reference lives: They only tell the compiler how the lifetimes of different references relate to each other.
		- Lifetime elision rules: Rust has rules for eliding (omitting) lifetimes when the relationships are clear, reducing the need for explicit annotations in many cases.
		- Multiple references with different lifetimes: You can have multiple references with different lifetimes, but the compiler ensures they don’t overlap in ways that would cause errors.
*/


// Basic Lifetime example
	// Explanation:
		// The function longest takes two string slices (&str) and returns the longest one.
		// The 'a lifetime annotation ensures that the returned reference lives as long as the shortest of the two references.
		// Without this explicit annotation, Rust wouldn't know how long the returned reference should be valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}


// Lifetime Elision Rule
	// Explanation:
		// This function finds the first word of a string and returns a slice.
		// Lifetime Elision Rule: Rust infers lifetimes automatically, meaning the function is treated as:
fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}


// Structs with Lifetimes
	// Explanation:
		// If a struct contains references, lifetime annotations are required to ensure the references remain valid.
		// The 'a annotation guarantees that the reference inside the struct does not outlive the data it refers to.
struct ImportantExcerpt<'a> {
	part: &'a str,
}


// Lifetime in Methods
	// Explanation:
	// 	Methods on a struct automatically inherit the struct’s lifetime ('a).
	// 	The function level does not return any reference, so it does not require additional lifetimes
impl<'a> ImportantExcerpt<'a> {
	fn level(&self) -> i32 {
		3
	}

	// Returning a reference with the same lifetime
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


// The 'static Lifetime
	// Explanation:
		// 'static lifetime means "lives for the entire duration of the program".
		// String literals ("...") have a static lifetime because they are stored in the binary and exist throughout execution.
fn static_str() -> &'static str {
	"I have a static lifetime."
}


// Multiple Lifetimes in Function Signatures - Dangling References
	// Explanation:
		// The function takes two string slices and returns a reference to a string slice.
		// The lifetimes of the two string slices are different, so the function returns a reference with a lifetime that is different from the input lifetimes.
fn longests<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
	'b: 'a,  // This ensures that 'b' outlives 'a'
{
	if x.len() > y.len() {
		x
	} else {
		y
	}
}



// Combining Lifetimes with Generics
	// Explanation:
		// The function can accept a reference to any type (T) that implements Display.
		// The lifetime annotation 'a ensures item is valid as long as needed.
fn print_with_lifetime<'a, T>(item: &'a T)
where
	T: std::fmt::Display,
{
	println!("{}", item);
}


fn main() {
	// Basic Lifetime example
	let string1 = String::from("Hello");
	let string2 = "world";

	let result = longest(&string1, &string2);
	println!("The longest string is {}", result);


	// Lifetime Elision Rule
	let string3 = "Hello world";
	let result2 = first_word(&string3);
	println!("The first word is {}", result2);

	// Structs with Lifetimes
	let novel = String::from("Call me Ishmael. Some years ago...");
	let first_sentence = novel.split('.').next().expect("Could not find a '.'");
	let excerpt = ImportantExcerpt { part: first_sentence };

	println!("{}", excerpt.part);

	// Lifetime in Methods
	let novel = String::from("Call me Ishmael.");
	let first_sentence = novel.split('.').next().unwrap();
	let excerpt = ImportantExcerpt { part: first_sentence };

	println!("Level: {}", excerpt.level());

	// The 'static Lifetime
	let s: &'static str = static_str();
	println!("{}", s);

	// Multiple Lifetimes in Function Signatures
	let string1 = String::from("Hello");
	let string2 = String::from("world");

	let result = longests(&string1, &string2);
	println!("The longest string is {}", result);

	// Combining Lifetimes with Generics
	let number = 42;
	print_with_lifetime(&number);
}