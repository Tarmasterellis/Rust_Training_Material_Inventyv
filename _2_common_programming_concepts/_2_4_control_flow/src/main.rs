/* Control Flow

	Control Flow are of two types:
		1. Conditional Control Flow: Control flow that is based on a condition.
		2. Unconditional Control Flow: Control flow that is not based on a condition.

	Conditional Control Flow
		1. if Statements
			- if Statements are used to conditionally execute code based on a condition.
			- if Statements have the following syntax:
				- if <condition> {
					- <code>
				- }
			- if Statements can be nested inside other if Statements.
			- if Statements can be used to conditionally execute code based on a condition.
		2. else Statements
			- else Statements are used to conditionally execute code based on a condition.
			- else Statements have the following syntax:
				- if <condition> {
					- <code>
				- } else {
					- <code>
				}
			- else Statements can be nested inside other else Statements.
			- else Statements can be used to conditionally execute code based on a condition.
		3. else if Statements
			- else if Statements are used to conditionally execute code based on a condition.
			- else if Statements have the following syntax:
				- if <condition> {
					- <code>
				- } else if <condition> {
					- <code>
				}
			- else if Statements can be nested inside other else if Statements.
			- else if Statements can be used to conditionally execute code based on a condition.
		4. match Statements
			- match Statements are used to conditionally execute code based on a value.
			- match Statements have the following syntax:
				- match <value> {
					- <pattern> => <code>,
					- <pattern> => <code>,
					- ...
				}
			- match Statements can be nested inside other match Statements.
			- match Statements can be used to conditionally execute code based on a value.

	Unconditional Control Flow
		1. for Loops
			- for Loops are used to iterate over a range of values.
			- for Loops have the following syntax:
				- for <pattern> in <range> {
					- <code>
				}
			- for Loops can be nested inside other for Loops.
			- for Loops can be used to iterate over a range of values.
		2. while Loops
			- while Loops are used to iterate as long as a condition is true.
			- while Loops have the following syntax:
				- while <condition> {
					- <code>
				}
			- while Loops can be nested inside other while Loops.
			- while Loops can be used to iterate as long as a condition is true.
		3. Loop Control Statements
			- Loop Control Statements are used to control the flow of a loop.
			- Loop Control Statements have the following syntax:
				- break;
				- continue;
			- Loop Control Statements can be used to control the flow of a loop.
			- break; is used to exit a loop prematurely.
			- continue; is used to skip the rest of the current iteration of a loop and move on to the next iteration.
*/

fn main() {
	// Conditional Control Flow
	let x = 5;

	if x > 0 {
		println!("x is positive");
	} else if x < 0 {
		println!("x is negative");
	} else {
		println!("x is zero");
	}

	// Unconditional Control Flow
	let mut i = 1;

	loop {
		println!("i: {}", i);

		if i == 10 {
			break;
		}

		i += 1;
	}

	// Loop Control Statements
	let mut i = 1;

	loop {
		println!("i: {}", i);

		if i == 10 {
			break;
		}

		i += 1;
	}

	i = 1;

	'outer: loop {
		println!("i: {}", i);

		if i == 10 {
			break;
		}

		i += 1;

		'inner: loop {
			println!("i: {}", i);

			if i == 10 {
				break 'outer;
			}

			i += 1;
			continue 'inner;
		}
	}

	match x {
		0 => println!("x is zero"),
		_ => println!("x is not zero"),
	}

	// for Loops
	for i in 1..=1000 {
		println!("i: {}", i);
	}

	// while Loops
	let mut i = 1;

	while i <= 5 {
		println!("i: {}", i);
		i += 1;
	}
}


