fn divide(a: i32, b: i32) -> Option<i32> {
	if b == 0 {
		None
	} else {
		Some(a / b)
	}
}

fn main() {
	let x = 10;
	let y = 0;
	match divide(x, y) {
		Some(result) => println!("Result: {}", result),
		None => println!("Cannot divide by zero!"),
	}


	println!("Starting hello_debug...");

	let numbers = vec![10, 20, 30, 40];
	let idx = 5; // intentionally out-of-bounds to trigger a panic for debugging
	println!("Requesting index {}", idx);

	let result = compute(numbers, idx);
	println!("Result: {}", result);
}



fn compute(v: Vec<i32>, i: usize) -> i32 {
	// small computation to step through
	let mut acc = 0;
	for (j, &val) in v.iter().enumerate() {
		acc += val;
		if j == i {
			// this branch won't run because i is out-of-range --
			// it shows a place to set a breakpoint and step in.
			return val * 2;
		}
	}
	// This next line will panic on out-of-bounds if uncommented:
	// v[i]
	acc
}