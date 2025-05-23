/*
    Rc<T> (Reference Counting)
        - Rc<T> is a single-threaded reference-counted smart pointer that allows multiple parts of a program to share ownership of the same data. It keeps track of how many references exist to the same value and deallocates the data when no references remain.
        - Key Features of Rc<T>:
            - Allows multiple ownership of data.
            - Works only in a single-threaded context (not thread-safe).
            - Uses non-atomic reference counting (faster than Arc<T> but unsafe for concurrency).

    Arc<T> (Atomic Reference Counting)
        - Arc<T> is a thread-safe reference-counted smart pointer that allows multiple threads to share ownership of the same data. Unlike Rc<T>, it uses atomic operations to ensure safety in concurrent environments.
        - Key Features of Arc<T>:
            - Allows multiple ownership across multiple threads.
            - Uses atomic reference counting for thread safety.
            - Introduces performance overhead due to atomic operations.
*/


use std::thread;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock}; // Ensure RwLock is imported

fn main() {
    // Rc<T> Example (Single-threaded Reference Counting)
    let shared_data = Rc::new(42); // Create a reference-counted integer

    let ref1 = Rc::clone(&shared_data); // Clone increases the reference count
    let ref2 = Rc::clone(&shared_data); // Another clone

    println!("Reference Count: {}", Rc::strong_count(&shared_data));
    println!("ref1: {}, ref2: {}", ref1, ref2);

    // Arc<T> Example (Thread-Safe Reference Counting)
    let shared_data = Arc::new(100); // Wrap data inside an Arc
    let mut handles = vec![];

    for _ in 0..3 { // Using more than 3 threads
        let shared_data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            println!("Thread {:?}: Value = {}", thread::current().id(), shared_data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Arc<T> with Mutex<T> for Mutable Shared State
    let counter = Arc::new(Mutex::new(0)); // Shared counter
    let mut handles = vec![];

    for _ in 0..4 { // Using 4 threads
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Lock the mutex
            *num += 1; // Modify the shared value
            println!("Thread {:?}: Counter = {}", thread::current().id(), *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Counter Value: {}", *counter.lock().unwrap());

    // Arc<T> with RwLock<T> for Better Performance
    let data = Arc::new(RwLock::new(0)); // Shared read-write data
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_val = *data.read().unwrap(); // Read lock
            println!("Thread {:?} Read: {}", thread::current().id(), read_val);
        });
        handles.push(handle);
    }

    {
        let mut write_data = data.write().unwrap(); // Write lock
        *write_data = 42; // Modify the shared data
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Value: {}", *data.read().unwrap());
}
