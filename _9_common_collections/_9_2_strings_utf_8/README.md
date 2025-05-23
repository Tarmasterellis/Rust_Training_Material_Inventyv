# Strings - UTF-8

- In Rust, the `String` type is a heap-allocated, growable string that is guaranteed to be valid UTF-8 encoded data. This means that `String` can store any sequence of characters that are part of the Unicode standard, which includes a vast set of characters from different languages, symbols, and emojis. Rust's `String` type is built on top of a `Vec<u8>`, meaning it stores bytes, but it enforces that these bytes are valid UTF-8.

- Let's break down the concept of UTF-8 and how Rust handles it in its `String` type with examples and different ways to work with it.

1. **UTF-8 Basics**
- UTF-8 is a variable-width character encoding that can represent every character in the Unicode standard. It uses 1 to 4 bytes to encode characters, where:
	1. ASCII characters (like `A`, `B`, `1`, etc.) are 1 byte.
	2. Other characters (like `é`, `ç`, or `漢`) require more bytes.

- In Rust, all `String` and `&str` types are valid UTF-8, ensuring that they can represent any Unicode character.

2. **Creating a UTF-8 `String` in Rust**

- You can create UTF-8 strings using different methods. Rust ensures that these strings are valid UTF-8 encoded at all times.

	- Using String::from()

		- You can create a String from a string literal (which is also valid UTF-8).

		```rust
		fn main() {
			let hello = String::from("Hello, 世界!"); // Unicode characters (世界) and ASCII
			println!("{}", hello);
		}
		```

		- Here, `String::from("Hello, 世界!")` creates a `String` that includes both ASCII characters and Chinese characters, demonstrating that Rust strings can hold Unicode.

	- Using `to_string()`
		
		- You can also convert string literals directly to `String` using `.to_string()`:

		```rust
		fn main() {
			let emoji = "😊 Rust".to_string();
			println!("{}", emoji);
		}
		```

		- This converts the string literal `"😊 Rust"` (which includes an emoji) into a UTF-8 encoded `String`.

	- Using `String::new()`

		- You can create an empty `String` and populate it later:

		```rust
		fn main() {
			let mut s = String::new(); // Empty string
			s.push_str("Hello, 世界!");
			println!("{}", s);
		}
		```

		- This creates an empty string and then adds the UTF-8 encoded string `"Hello, 世界!"` to it.

3. **UTF-8 Encoding and Byte Representation**
- Rust strings are UTF-8 under the hood, which means characters are stored as a series of bytes. You can inspect the underlying bytes of a String:

	```rust
	fn main() {
		let s = String::from("Hola! 😊");
		for b in s.as_bytes() {
			print!("{} ", b); // Prints the UTF-8 bytes
		}
	}
	```
	- Output:
	```shell
	72 111 108 97 33 32 240 159 152 138
	```

	- The ASCII characters are 1 byte each (e.g., `H` = `72`), but the emoji `😊` is encoded in 4 bytes (`240 159 152 138`).

4. **Handling Characters vs. Bytes**
- Since Rust strings are UTF-8 encoded, the number of bytes might not equal the number of characters. For example, an emoji or non-ASCII character could take more than 1 byte. Rust provides different ways to work with these.

	- Iterating Over Characters

		- To iterate over characters (not bytes):

			```rust
			fn main() {
				let s = String::from("Hola! 😊");
				for c in s.chars() {
					println!("{}", c);
				}
			}
			```
			- Output:
			```css
			H
			o
			l
			a
			!
			
			😊
			```
		- The `chars()` method returns an iterator over each Unicode character.

	- Iterating Over Bytes
		- To iterate over the raw bytes of the string:

		```rust
		fn main() {
			let s = String::from("Hola! 😊");
			for b in s.bytes() {
				println!("{}", b);
			}
		}
		```
		- Output:
		```shell
		72
		111
		108
		97
		33
		32
		240
		159
		152
		138
		```
		- This shows the raw UTF-8 byte values of each character, including the multi-byte encoding of `😊`.

5. **String Slicing and UTF-8 Safety**
- Since Rust strings are UTF-8 encoded, slicing them by byte indices can be dangerous because you might accidentally cut a character in the middle of its multi-byte representation.

- Rust enforces this by only allowing slicing by character boundaries:

	```rust
	fn main() {
		let s = String::from("Hola! 😊");
		
		// This works because it's slicing at character boundaries
		let slice = &s[0..5]; // "Hola!"
		println!("{}", slice);
		
		// This would cause a panic if you try to slice in the middle of a multi-byte character
		// let invalid_slice = &s[0..7]; // Panics at runtime
	}
	```
- Rust will panic if you try to slice a string at a point that is not a valid UTF-8 boundary. In the example above, slicing at 7 would cut into the multi-byte `😊` character and thus is not allowed.

6. **Converting Between Strings and Bytes**
- You can convert `String` into a `Vec<u8>` and vice versa, but you must ensure that any bytes are valid UTF-8 when converting back to String.

	- Converting String to `Vec<u8>`(Bytes)

		```rust
		fn main() {
			let s = String::from("Hello!");
			let bytes = s.into_bytes(); // Converts into a Vec<u8>
			println!("{:?}", bytes); // [72, 101, 108, 108, 111, 33]
		}
		```

	- Converting `Vec<u8>`to String

	```rust
	fn main() {
		let bytes = vec![72, 101, 108, 108, 111, 33];
		let s = String::from_utf8(bytes).unwrap(); // Converts bytes to a String
		println!("{}", s); // "Hello!"
	}
	```
- If the byte sequence is not valid UTF-8, the conversion will fail and return a `Result` with an error.


### Conclusion:
- Rust’s String is a UTF-8 encoded growable string that can hold any Unicode character.
- UTF-8 is variable-width (1-4 bytes), so some characters take more space than others.
- You can manipulate strings by characters or bytes, but slicing by byte indices must respect UTF-8 boundaries.
- Rust provides methods like chars(), bytes(), and as_bytes() to work with different aspects of String.


### Note:-
- Rust’s emphasis on ensuring that strings are valid UTF-8 helps ensure correctness and safety when working with text data.