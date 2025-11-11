/*
    Property-Based Testing:
        - Property-based testing (PBT) is a software testing method that focuses on testing a specific property or invariant of a software system.
        - It is based on the idea that a property is a specific characteristic or behavior of a system that should hold true for all valid inputs.
        - PBT is often used to test the correctness of algorithms, data structures, and other software components.
        - PBT can be used to test the correctness of code that is difficult to test using traditional testing methods.
        - PBT can be used to test code that is difficult to test using traditional testing methods.
*/

use proptest::prelude::*;

/// ### Example 4: Testing Async Functions
#[tokio::test]
async fn test_async_function() {
    async fn fetch_data() -> u32 {
        42
    }
    assert_eq!(fetch_data().await, 42);
}

/// ## Property-Based Testing in Rust
/// Property-based testing tests a function against a range of inputs.


/// ### Example 1: Basic Property Test
proptest! {
    #[test]
    fn test_addition_commutative(a in 0..1000i32, b in 0..1000i32) {
        assert_eq!(a + b, b + a);
    }
}

/// ### Example 2: Testing Sorting Properties
proptest! {
    #[test]
    fn test_sorting(mut v in proptest::collection::vec(0..100i32, 0..100)) {
        v.sort();
        for i in 1..v.len() {
            assert!(v[i - 1] <= v[i]);
        }
    }
}

/// ### Example 3: Testing String Properties
proptest! {
    #[test]
    fn test_string_length(s in "[a-zA-Z]{1,100}") {
        assert!(s.len() <= 100);
    }
}

/// ### Example 4: Arbitrary Data Structures
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Arbitrary for Point {
    type Parameters = ();
    type Strategy = BoxedStrategy<Point>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (0..1000, 0..1000)
            .prop_map(|(x, y)| Point { x, y })
            .boxed()
    }
}

proptest! {
    #[test]
    fn test_point_values(p in any::<Point>()) {
        assert!(p.x >= 0 && p.y >= 0);
    }
}