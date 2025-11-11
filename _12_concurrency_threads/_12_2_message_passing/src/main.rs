/*
    Message Passing:
        - Message passing is a communication mechanism where a sender sends a message to a receiver.
        - It is a synchronous communication mechanism, meaning that the sender will block until the receiver has received the message.
        - Message passing can be used to synchronize threads, share data, and coordinate actions among multiple threads.

    Channels:
        - Channels are a type of message passing that allow multiple threads to send and receive messages.
        - They provide a way to communicate between threads without the need for shared memory or locks.
        - Channels can be used to implement synchronization, data sharing, and communication between threads.

    Mutex:
        - Mutex is a synchronization primitive that allows only one thread to access a shared resource at a time.
        - It provides exclusive access to the shared resource, preventing other threads from accessing it simultaneously.
        - Mutex is typically used to protect shared data from concurrent access by multiple threads.

    RwLock:
        - RwLock is a synchronization primitive that allows multiple threads to access a shared resource simultaneously.
        - It provides shared access to the shared resource, allowing multiple threads to read or modify the data concurrently.
            However, it also provides exclusive access to the resource, preventing other threads from accessing it simultaneously.
        - RwLock is typically used to protect shared data from concurrent access by multiple threads.

    Arc<Mutex<T>>:
        - Arc<Mutex<T>> is a thread-safe version of Mutex<T>.
        - It provides shared access to the shared resource, allowing multiple threads to read or modify the data concurrently.
            However, it also provides exclusive access to the resource, preventing other threads from accessing it simultaneously.
        - Arc<Mutex<T>> is typically used to protect shared data from concurrent access by multiple threads.

    Arc<RwLock<T>>:
        - Arc<RwLock<T>> is a thread-safe version of RwLock<T>.
        - It provides shared access to the shared resource, allowing multiple threads to read or modify the data concurrently.
            However, it also provides exclusive access to the resource, preventing other threads from accessing it simultaneously.
        - Arc<RwLock<T>> is typically used to protect shared data from concurrent access by multiple threads.

    mpsc (Multiple Producer, Single Consumer):
        - mpsc is a channel type that allows multiple threads to send messages to a single consumer thread.
        - It provides a way to communicate between threads without the need for shared memory or locks.
        - mpsc is typically used to implement message passing between threads.
    
    mpsc::channel():
        - mpsc::channel() returns a tuple of two channels: a sender and a receiver.
        - The sender can be used to send messages to the receiver.
        - The receiver can be used to receive messages from the sender.
        - The sender and receiver can be cloned and sent to multiple threads.

    mpsc::sync_channel():
        - mpsc::sync_channel() returns a tuple of two channels: a sender and a receiver.
        - The sender can be used to send messages to the receiver.
        - The receiver can be used to receive messages from the sender.
        - The sender and receiver can be cloned and sent to multiple threads.
        - The sender and receiver are thread-safe, meaning that they can be used to send and receive messages simultaneously by multiple threads.
            However, they also provide exclusive access to the channel, preventing other threads from accessing it simultaneously.
        - The sender and receiver are typically used to implement message passing between threads in a concurrent environment.
*/

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock};


fn main() {

    // /// Using mpsc (Multiple Producer, Single Consumer)
    // // Create a channel
    // let (tx, rx) = mpsc::channel();

    // // Spawn a new thread
    // thread::spawn(move || {
    //     let messages = vec![
    //         String::from("Hello"),
    //         String::from("from"),
    //         String::from("Rust"),
    //         String::from("Thread!"),
    //     ];

    //     for msg in messages {
    //         tx.send(msg).unwrap(); // Send message through the channel
    //         thread::sleep(Duration::from_millis(500)); // Simulate work
    //     }
    // });

    // // Receive messages in the main thread
    // for received in rx {
    //     println!("Received: {}", received);
    // }


    // /// Using Arc<Mutex<T>> for Shared State
    
    // let counter = Arc::new(Mutex::new(0)); // Shared mutable state inside an Arc<Mutex<T>>
    // let mut handles = vec![];

    // for _ in 0..5 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap(); // Lock the Mutex
    //         *num += 1; // Modify the shared counter
    //         println!("Thread {:?}: Counter = {}", thread::current().id(), *num);
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Final counter value: {}", *counter.lock().unwrap());


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
    let readers: Vec<_> = (0..20).map(|_| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
            let read_data = data.read().unwrap(); // Read lock
            println!("Reader Thread: {:?}", *read_data);
        })
    }).collect();

    // Multiple writer threads (more than 3 writers)
    let writer2_threads: Vec<_> = (6..10).map(|val| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
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