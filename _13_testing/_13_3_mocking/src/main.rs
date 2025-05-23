/*
    Mocking:
        - Mocking is a technique used in software testing to simulate the behavior of a real object or component.
        - It allows developers to isolate the code being tested and focus on testing specific functionality without relying on external dependencies.
        - Mocking frameworks, such as Mockito in Java, provide a way to define and configure mock objects.
        - Mock objects can be used to simulate the behavior of real objects or components during testing.
        - Mock objects can be used to verify that the code being tested interacts correctly with the mock object.
*/

/// ### Example 3: Testing with File I/O
#[test]
fn test_file_io() {
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("test.txt").unwrap();
    writeln!(file, "Hello, world!").unwrap();
    assert!(std::path::Path::new("test.txt").exists());
}

/// ## Mocking in Rust
/// Mocking allows replacing real dependencies with test-specific implementations.

/// ### Example 1: Simple Mock
trait Greeter {
    fn greet(&self) -> String;
}

struct MockGreeter;

impl Greeter for MockGreeter {
    fn greet(&self) -> String {
        "Hello, Mock!".to_string()
    }
}

#[test]
fn test_mock() {
    let greeter = MockGreeter;
    assert_eq!(greeter.greet(), "Hello, Mock!");
}

/// ### Example 2: Using Mockall crate
#[cfg(test)]
mod mock_tests {
    use mockall::{automock, predicate::*};

    #[automock]
    trait Service {
        fn fetch_data(&self) -> String;
    }

    #[test]
    fn test_mockall() {
        let mut mock = MockService::new();
        mock.expect_fetch_data().returning(|| "Mocked Data".to_string());
        assert_eq!(mock.fetch_data(), "Mocked Data");
    }
}

/// ### Example 3: Mocking External API Calls
#[test]
fn test_mock_external_call() {
    struct HttpClient;
    impl HttpClient {
        fn get(&self, _url: &str) -> String {
            "Mock Response".to_string()
        }
    }
    let client = HttpClient;
    assert_eq!(client.get("http://example.com"), "Mock Response");
}

/// ### Example 4: Mocking with Closures
fn process<F: Fn() -> i32>(f: F) -> i32 {
    f() + 1
}

#[test]
fn test_mock_closure() {
    let mock_fn = || 10;
    assert_eq!(process(mock_fn), 11);
}