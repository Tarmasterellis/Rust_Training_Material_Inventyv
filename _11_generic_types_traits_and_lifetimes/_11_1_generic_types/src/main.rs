/*
    Generic Types
        - Generic types are types that can be parameterized with other types.
        - They are defined using angle brackets (<>) and can be used in function signatures, struct definitions, and enum definitions.
        - Generic types can be used in function signatures to create functions that can work with different types.
        - Generic types can be used in struct definitions to create structs that can work with different types.
        - Generic types can be used in enum definitions to create enums that can work with different types.
*/

// Generic types can be used in function signatures to create functions that can work with different types.
fn get_larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Generic types can be used in struct definitions to create structs that can work with different types.
struct Point<T> {
    x: T,
    y: T,
}


struct Points<T, U> {
    x: T,
    y: U,
}


// Generics can be used in enum definitions to create enums that can work with different types.
#[derive(Debug)]
#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None,
}


fn main() {
    let larger_int = get_larger(3, 5);
    let larger_float = get_larger(2.5, 7.5);
    
    println!("Larger integer: {}", larger_int);
    println!("Larger float: {}", larger_float);

    // Generic types can be used in struct definitions to create structs that can work with different types and one Generic type.
    let int_point = Point { x: 3, y: 5 };
    let float_point = Point { x: 2.5, y: 7.5 };

    println!("Point: ({}, {})", int_point.x, int_point.y);
    println!("Point: ({}, {})", float_point.x, float_point.y);

    // Generic types can be used in struct definitions to create structs that can work with different types and Different Generic type.
    let point = Points { x: 5, y: 1.5 };

    println!("Mixed Point: ({}, {})", point.x, point.y);


    // Generics can be used in enum definitions to create enums that can work with different types.
    let some_number = Option::Some(5);
    let some_string = Option::Some("Hello");
    
    println!("Number: {:?}", some_number);
    println!("String: {:?}", some_string);
}