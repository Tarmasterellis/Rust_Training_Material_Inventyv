/* Error Handling using Panic */

fn main() {
	let v = vec![1, 2, 3];

    // This will panic because we're accessing out of bounds
    println!("{}", v[99]);
}