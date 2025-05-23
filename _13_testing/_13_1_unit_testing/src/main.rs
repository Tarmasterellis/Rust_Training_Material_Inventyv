/*
	Unit Testing:
		- Unit testing is a software testing method by which individual units of source code, sets of one or more computer program modules together with associated control data, usage procedures, and operating procedures, are tested to determine whether they are fit for use.
		- Intention is to validate that each unit of the software performs as designed.
		- A unit is the smallest testable part of any software.
		- Unit tests are typically written by the developers themselves.
		- The tests are run automatically to ensure that the software is functioning as expected.
		- Test-driven development (TDD) is a software development process that relies on writing tests before writing the actual code.
		- TDD is a way of writing code that is easier to test and maintain.
*/

/// ## Unit Testing in Rust
/// Unit tests focus on testing small, isolated parts of the code.
///
/// ### Example 1: Basic Unit Test
#[cfg(test)]
mod tests {
	fn add(a: i32, b: i32) -> i32 {
		a + b
	}

	#[test]
	fn test_add() {
		assert_eq!(add(2, 3), 5);
	}
}

/// ### Example 2: Testing for Panic
#[test]
#[should_panic]
fn test_panic() {
	panic!("This should panic");
}

/// ### Example 3: Using `Result` in Tests
#[test]
fn test_result() -> Result<(), String> {
	if 2 + 2 == 4 {
		Ok(())
	} else {
		Err("Math is broken!".to_string())
	}
}

/// ### Example 4: Table-driven tests
fn multiply(a: i32, b: i32) -> i32 {
	a * b
}

#[test]
fn test_multiply() {
	let cases = vec![(2, 3, 6), (-1, 5, -5), (0, 10, 0)];
	for (a, b, expected) in cases {
		assert_eq!(multiply(a, b), expected);
	}
}