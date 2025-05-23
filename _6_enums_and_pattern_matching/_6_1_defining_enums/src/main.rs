/*
    Defining Enums in Rust

    - Enums allow defining a type that can have a fixed set of values.
    - Useful for defining multiple related types under a single structure.
    - Variants can be simple, hold data, or reference other types.

    Example:
        - Basic Enums: `Color` and `Size` with fixed values.
        - Complex Enums: `Message`, which can store different data types.
*/

fn main() {
    let x = Color::Red;
    let y = Color::Blue;
    let z = Color::Green;

    println!("{:?}", x);  // Red
    println!("{:?}", y);  // Blue
    println!("{:?}", z);  // Green

    let x = Size::Small;
    let y = Size::Medium;
    let z = Size::Large;

    println!("{:?}", x);  // Small
    println!("{:?}", y);  // Medium
    println!("{:?}", z);  // Large

    let message = Message::Move { x: 5, y: 10 };

    // Using match instead of if let
    match message {
        Message::Move { x, y } => println!("Moving to coordinates: x: {}, y: {}", x, y),
        _ => println!("{:?}", message),
    }

    let messages = [
        Message::Quit,
        Message::Write(String::from("Hello")),
        Message::ChangeColor(Color::Red),
        Message::ChangeSize(Size::Small),
    ];

    for msg in messages.iter() {
        msg.process();
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
    ChangeSize(Size),
}

// Implementation block for Message
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to coordinates: x: {}, y: {}", x, y),
            Message::Write(text) => self.write_message(text),
            Message::ChangeColor(color) => self.change_color(color),
            Message::ChangeSize(size) => self.change_size(size),
        }
    }

    fn write_message(&self, text: &String) {
        println!("Writing message: {}", text);
    }

    fn change_color(&self, color: &Color) {
        println!("Changing color to {:?}", color);
    }

    fn change_size(&self, size: &Size) {
        println!("Changing size to {:?}", size);
    }
}
