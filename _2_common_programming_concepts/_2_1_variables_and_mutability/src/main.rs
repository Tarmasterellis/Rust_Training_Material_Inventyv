/* Variables and Mutability

	Variables
		1. Variables are used to store data that can be used later in the program.
		2. Variables can be either mutable or immutable.
		3. Variables can be reassigned to a new value (Only if the variable is mutable).
		4. Variables can be shadowed by other variables with the same name.
		5. Variables have a lifetime that determines how long they can be used in the program.
	
	Mutability
		1. Variables can be either mutable or immutable.
		2. Mutable variables can be changed after they are assigned a value.
		3. Immutable variables cannot be changed after they are assigned a value.
	
	Shadowing
		1. Variables can be shadowed by other variables with the same name.
		2. This can lead to unexpected behavior and bugs in the program.

	Variable Lifetime
		1. Variables have a lifetime that determines how long they can be used in the program.
		2. There are two types of variable lifetime in Rust:
			- Static: Variables declared outside of any function or block have a static lifetime and can be accessed from anywhere in the program.
			- Dynamic: Variables declared inside a function or block have a dynamic lifetime and can only be accessed within that function or block.	
*/

fn main() {
	let x = 5;
	println!("The value of x is: {x}");

	let mut y = 10;
	println!("The value of y is: {y}");

	y = 20;
	println!("The value of y is: {y}");

	let z = 30;
	println!("The value of z is: {z}");

	let z = 40;
	println!("The value of z is: {z}");
}