# Concurrency (Threads), Arc, Variable Lock (RwLock, OneCell), Mesage Passing

## Concurrency (Threads)

### Threads

- Threads are lightweight processes that can run concurrently with other threads.
- Threads are used to perform tasks concurrently, allowing multiple tasks to be executed simultaneously.
- Threads are used to perform tasks concurrently, allowing multiple tasks to be executed simultaneously.
- Rust provides a built-in library for working with threads, called the std::thread module.
- The std::thread module provides functions for creating and managing threads.
- The std::thread module provides functions for creating and managing threads.

#### Creating a Thread

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

#### Threads and Ownership

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let x = 5;
        println!("x is: {}", x);
    });

    handle.join().unwrap();
}
```

## Arc

### Arc

- Arc is a smart pointer that allows sharing ownership of a value between multiple threads.

#### Creating an Arc

```rust
use std::sync::Arc;

fn main() {
    let x = Arc::new(5);
    let y = x.clone();

    println!("x is: {}", x);
    println!("y is: {}", y);
}
```

#### Arc and Ownership

```rust
use std::sync::Arc;

fn main() {
    let x = Arc::new(5);
    let y = x.clone();

    println!("x is: {}", x);
    println!("y is: {}", y);
}
```

## Variable Lock (RwLock, OneCell)

### Variable Lock

- Variable locks are used to synchronize access to shared data.
- They allow multiple threads to access shared data concurrently without causing data races.
- Variable locks are implemented using the Mutex type from the std::sync module.
- The Mutex type provides methods for acquiring and releasing locks.
- The Mutex type is generic, meaning it can be used with any type that implements the Send trait.
- The Mutex type is used to protect shared data from concurrent access.
- The Mutex type is used to synchronize access to shared data.
- The Mutex type is used to
    - acquire a lock on shared data
    - release a lock on shared data
    - access shared data

### RwLock

- RwLock is a type of lock that allows multiple readers or a single writer to access shared data concurrently.

#### Creating a RwLock

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);
    let read_guard = lock.read().unwrap();
    let write_guard = lock.write().unwrap();

    println!("Read guard: {}", read_guard);
    println!("Write guard: {}", write_guard);
}
```

#### RwLock and Ownership

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);
    let read_guard = lock.read().unwrap();
    let write_guard = lock.write().unwrap();

    println!("Read guard: {}", read_guard);
    println!("Write guard: {}", write_guard);
}
```

### OneCell

- OneCell is a type of lock that allows only one thread to access shared data at a time.

#### Creating a OneCell

```rust
use std::cell::{Cell, RefCell, Ref, RefMut};

fn main() {
    let x = Cell::new(5);
    let y = RefCell::new(5);
    let z = Ref::map(y.borrow(), |r| r + 1);
    let w = RefMut::map(y.borrow_mut(), |r| *r + 1);

    println!("x is: {}", x.get());
    println!("y is: {}", y.borrow());
    println!("z is: {}", z);
    println!("w is: {}", w);
}
```

#### OneCell and Ownership

```rust
use std::cell::{Cell, RefCell, Ref, RefMut};

fn main() {
    let x = Cell::new(5);
    let y = RefCell::new(5);
    let z = Ref::map(y.borrow(), |r| r + 1);
    let w = RefMut::map(y.borrow_mut(), |r| *r + 1);

    println!("x is: {}", x.get());
    println!("y is: {}", y.borrow());
    println!("z is: {}", z);
    println!("w is: {}", w);
}
```

## Mesage Passing

### Message Passing

- Message passing is a way of sharing data between threads.
- It allows threads to communicate with each other by sending and receiving messages.   
- Message passing is useful when you want to share data between threads without using shared memory.   
- Message passing is a fundamental concept in concurrent programming.

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // Example 1: Sending multiple messages from a single thread
    let (sender1, receiver1) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for message in messages {
            sender1.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in receiver1 {
        println!("Received message: {}", received);
    }

    handle1.join().unwrap();

    // Example 2: Sending messages from multiple threads
    let (sender2, receiver2) = mpsc::channel();
    let sender2_clone = sender2.clone();

    let handle2 = thread::spawn(move || {
        sender2.send("Hello from thread 1").unwrap();
    });

    let handle3 = thread::spawn(move || {
        sender2_clone.send("Hello from thread 2").unwrap();
    });

    for _ in 0..2 {
        let received = receiver2.recv().unwrap();
        println!("Received message: {}", received);
    }

    handle2.join().unwrap();
    handle3.join().unwrap();

    // Example 3: Using a channel with a custom data type
    let (sender3, receiver3) = mpsc::channel();

    let handle4 = thread::spawn(move || {
        let data = vec![1, 2, 3, 4, 5];
        sender3.send(data).unwrap();
    });

    let received_data = receiver3.recv().unwrap();
    println!("Received data: {:?}", received_data);

    handle4.join().unwrap();
}
```

### Message Passing with Ownership

```rust
use std::thread;    
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // Example 1: Sending multiple messages from a single thread
    let (sender1, receiver1) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for message in messages {
            sender1.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in receiver1 {
        println!("Received message: {}", received);
    }

    handle1.join().unwrap();

    // Example 2: Sending messages from multiple threads
    let (sender2, receiver2) = mpsc::channel();
    let sender2_clone = sender2.clone();

    let handle2 = thread::spawn(move || {
        sender2.send("Hello from thread 1").unwrap();
    });

    let handle3 = thread::spawn(move || {
        sender2_clone.send("Hello from thread 2").unwrap();
    });

    for _ in 0..2 {
        let received = receiver2.recv().unwrap();
        println!("Received message: {}", received);
    }

    handle2.join().unwrap();
    handle3.join().unwrap();

    // Example 3: Using a channel with a custom data type
    let (sender3, receiver3) = mpsc::channel();

    let handle4 = thread::spawn(move || {
        let data = vec![1, 2, 3, 4, 5];
        sender3.send(data).unwrap();
    });

    let received_data = receiver3.recv().unwrap();
    println!("Received data: {:?}", received_data);

    handle4.join().unwrap();
}
```

## Conclusion
    - Concurrency is a powerful feature of Rust that allows you to write efficient and safe code.
    - Rust provides several concurrency primitives, including threads, message passing, and synchronization primitives.
    - Understanding concurrency is essential for writing high-performance and scalable applications.
    - Rust's concurrency model is based on the concept of ownership, which ensures that shared data is accessed safely and efficiently.
    