#[cfg(target_os = "linux")]
mod linux_only {
	pub fn linux_function() {
		println!("This only compiles on Linux");
	}
}

#[cfg(not(target_os = "linux"))]
mod non_linux {
	pub fn other_function() {
		println!("This compiles on non-Linux systems");
	}
}

fn main() {
	// linux_only::linux_function(); // use of undeclared crate or module `linux_only`
	non_linux::other_function();
}
