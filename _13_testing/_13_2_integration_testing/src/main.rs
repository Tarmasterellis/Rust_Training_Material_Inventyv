/*
    Integration Testing:
        - Integration testing is a software testing method that evaluates whether the system as a whole works correctly.
        - It focuses on testing the interactions between different components or modules of the software.
        - Integration tests are typically written by the developers themselves.
        - The tests are run automatically to ensure that the software is functioning as expected.
        - Test-driven development (TDD) is a software development process that relies on writing tests before writing the actual code.
        - TDD is a way of writing code that is easier to test and maintain.
*/

/// ## Integration Testing in Rust
/// Integration tests test the interaction of multiple parts of the system.

fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// ### Example 1: Simple Integration Test
// Place this in `tests/integration_test.rs`
#[test]
fn test_external_function() {
    assert_eq!(add(2, 3), 5);
}

/// ### Example 2: Testing with Environment Variables
#[test]
fn test_env_variable() {
    std::env::set_var("MY_VAR", "42");
    assert_eq!(std::env::var("MY_VAR").unwrap(), "42");
}