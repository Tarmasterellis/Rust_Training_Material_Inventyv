/*
    Rc<T> (Reference Counting)
        - Rc<T> is a single-threaded reference-counted smart pointer that allows multiple parts of a program to share ownership of the same data.
                It keeps track of how many references exist to the same value and deallocates the data when no references remain.
        - Key Features of Rc<T>:
            - Allows multiple ownership of data.
            - Works only in a single-threaded context (not thread-safe).
            - Uses non-atomic reference counting (faster than Arc<T> but unsafe for concurrency).

    Arc<T> (Atomic Reference Counting)
        - Arc<T> is a thread-safe reference-counted smart pointer that allows multiple threads to share ownership of the same data. Unlike Rc<T>,
                it uses atomic operations to ensure safety in concurrent environments.
        - Key Features of Arc<T>:
            - Allows multiple ownership across multiple threads.
            - Uses atomic reference counting for thread safety.
            - Introduces performance overhead due to atomic operations.
*/


use std::thread;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};
use std::sync::{Arc, Mutex, RwLock}; // Ensure RwLock is imported


// ///RC Example
// fn c<F: FnOnce() + 'static>(f: F) {
//     f();
// }

// fn main() {

//     let v = Arc::new(Mutex::new(vec![1, 2, 3]));

//     c({
//         let v = Arc::clone(&v);
//         move || {
//             println!("inner 1: {:?}", v);
//             v.lock().unwrap().push(4);
//         }
//     });

//     c({
//         let v = Arc::clone(&v);
//         move || {
//             println!("inner 2: {:?}", v);
//             v.lock().unwrap().push(5);
//         }
//     });


//     println!("outer: {:?}", v);
// }



//  Multi Threading Example
fn main() {
    // // Rc<T> Example (Single-threaded Reference Counting)
    // let shared_data = Rc::new(42); // Create a reference-counted integer

    // let ref1 = Rc::clone(&shared_data); // Clone increases the reference count
    // let ref2 = Rc::clone(&shared_data); // Another clone

    // println!("Reference Count Strong: {}", Rc::strong_count(&shared_data));
    // println!("Reference Count Weak: {}", Rc::weak_count(&shared_data));
    // println!("ref1: {}, ref2: {}", ref1, ref2);

    // // Arc<T> Example (Thread-Safe Reference Counting)
    // let shared_data = Arc::new(100); // Wrap data inside an Arc
    // let mut handles = vec![];

    // for _ in 0..3 { // Using more than 3 threads
    //     let shared_data = Arc::clone(&shared_data);
    //     let handle = thread::spawn(move || {
    //         println!("Thread {:?}: Value = {}", thread::current().id(), shared_data);
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // // Arc<T> with Mutex<T> for Mutable Shared State
    // let counter = Arc::new(Mutex::new(0)); // Shared counter
    // let mut handles = vec![];

    // for _ in 0..4 { // Using 4 threads
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap(); // Lock the mutex
    //         *num += 1; // Modify the shared value
    //         println!("Thread {:?}: Counter = {}", thread::current().id(), *num);
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Final Counter Value: {}", *counter.lock().unwrap());

    // Arc<T> with RwLock<T> for Better Performance
    let data = Arc::new(RwLock::new(0)); // Shared read-write data
    let mut handles = vec![];

    for _ in 0..1000 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(3));
            let read_val = *data.read().unwrap(); // Read lock
            println!("Thread {:?} Read: {}", thread::current().id(), read_val);
        });
        handles.push(handle);
    }

    for _ in 0..1 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            let mut write_data = data.write().unwrap(); // Write lock
            *write_data = 42; // Modify the shared data
        });
        handles.push(handle);
    }

    // {
    //     let mut write_data = data.write().unwrap(); // Write lock
    //     *write_data = 42; // Modify the shared data
    // }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Value: {}", *data.read().unwrap());
}