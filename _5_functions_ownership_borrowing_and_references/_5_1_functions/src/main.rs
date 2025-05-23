/* Functions with Statements and Expressions

    Functions:

        Functions are a fundamental building block of Rust programs. They allow you to encapsulate a block of code that performs a specific task, and then call that code from other parts of your program.

        In Rust, functions are defined using the `fn` keyword, followed by the function name, a set of parentheses containing the function's parameters, and a block of code enclosed in curly braces.

        Let's start by creating a function called `add` that takes two parameters, `a` and `b`, and returns their sum.

        We will then call the `add` function with two arguments and store the result in a variable called `result`.

        Finally, we will print the result.

    Statements and Expressions:
        Statements are instructions that perform an action and do not return a value. Examples of statements include variable declarations, function calls, and loop control statements.

        Expressions, on the other hand, are values that can be used in an expression. Examples of expressions include literals (such as numbers and strings), variable references, and function calls.
*/

// Defining a function called add that takes two parameters, a and b, and returns their sum
fn add(a: i32, b: i32) {
    println!("The sum of {} and {} is {}", a, b, a + b);
}

// // Function With Return Type
fn add_with_return_type(a: i32, b: i32) -> i32 {
    a + b
}


// Functions
fn main() {
	// Calling the add function with two arguments and storing the result in a variable called result
	add(5, 10);


    // Calling the add_with_return_type function with two arguments and storing the result in a variable called result
	let result = add_with_return_type(5, 10);

	// Printing the result
	println!("The sum of 5 and 10 is {}", result);

    // Statements
    let x = 3;
    let y = x + 1;

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    // Expressions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {:?}", y);
}