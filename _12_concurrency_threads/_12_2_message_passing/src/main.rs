/*
    Message Passing:
        - 
*/

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock};


fn main() {

    /// Using mpsc (Multiple Producer, Single Consumer)
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread
    thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("Rust"),
            String::from("Thread!"),
        ];

        for msg in messages {
            tx.send(msg).unwrap(); // Send message through the channel
            thread::sleep(Duration::from_millis(500)); // Simulate work
        }
    });

    // Receive messages in the main thread
    for received in rx {
        println!("Received: {}", received);
    }


    /// Using Arc<Mutex<T>> for Shared State
    
    let counter = Arc::new(Mutex::new(0)); // Shared mutable state inside an Arc<Mutex<T>>
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Lock the Mutex
            *num += 1; // Modify the shared counter
            println!("Thread {:?}: Counter = {}", thread::current().id(), *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());


    ///Using Arc<RwLock<T>> for Efficient Read-Write Access
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // First writer thread
    let writer = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut write_data = data.write().unwrap(); // Write lock
            write_data.push(4);
            println!("Writer Thread 1: {:?}", *write_data);
        })
    };

    // Reader threads (3 readers)
    let readers: Vec<_> = (0..3).map(|_| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let read_data = data.read().unwrap(); // Read lock
            println!("Reader Thread: {:?}", *read_data);
        })
    }).collect();

    // Multiple writer threads (more than 3 writers)
    let writer2_threads: Vec<_> = (6..10).map(|val| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut write_data = data.write().unwrap(); // Write lock
            write_data.push(val);
            println!("Writer Thread adding {}: {:?}", val, *write_data);
        })
    }).collect();

    // Wait for first writer to finish
    writer.join().unwrap();
    
    // Wait for reader threads to finish
    for reader in readers {
        reader.join().unwrap();
    }

    // Wait for multiple writer threads to finish
    for writer2 in writer2_threads {
        writer2.join().unwrap();
    }
    
}