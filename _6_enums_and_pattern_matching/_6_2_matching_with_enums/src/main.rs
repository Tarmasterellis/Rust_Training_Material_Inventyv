/* 
	Matching with Enums
		In Rust, enums are used to define types that can have multiple possible variants.
		Each variant of an enum can store different kinds of data.
		To handle enums, pattern matching is frequently used.
		The match and if let expressions are two common ways to match and destructure enum variants.


*/

// Here is a basic example with an enum Message and using match to handle different variants:
enum Message {
	Quit,  // No data associated with this variant
	Move { x: i32, y: i32 },  // Contains a struct-like pair of x and y coordinates
	Write(String),  // Contains a single String
	ChangeColor(Color),  // Contains another enum Color
}

#[derive(Debug)]
enum Color {
	Red,
	Green,
	Blue
}

fn process_message(msg: Message) {
	match msg {
		Message::Quit => {
			println!("The Quit message has no data.");
		},
		Message::Move { x, y } => {
			println!("Moving to coordinates: x = {}, y = {}", x, y);
		},
		Message::Write(text) => {
			println!("Text message: {}", text);
		},
		Message::ChangeColor(color) => {
			println!("Changing color to: {:?}", color);
		},
	}
}

fn main() {
	let msg1 = Message::Quit;
	let msg2 = Message::Move { x: 10, y: 20 };
	let msg3 = Message::Write(String::from("Hello, Rust!"));
	let msg4 = Message::ChangeColor(Color::Red);
	let msg5 = Message::ChangeColor(Color::Green);
	let msg6 = Message::ChangeColor(Color::Blue);

	process_message(msg1);  // The Quit message has no data.
	process_message(msg2);  // Moving to coordinates: x = 10, y = 20
	process_message(msg3);  // Text message: Hello, Rust!
	process_message(msg4);  // Changing color to: Red
	process_message(msg5);  // Changing color to: Green
	process_message(msg6);  // Changing color to: Blue
	
	
	// If you only care about one specific variant and want a simpler way to match it, you can use if let. Here's an example that checks only for the Message::Move variant:
	let msg = Message::Move { x: 5, y: 10 };

	if let Message::Move { x, y } = msg {
		println!("Moving to coordinates: x = {}, y = {}", x, y);
	} else {
		println!("Not a Move message");
	}


	// // Here is an example where enums store different types of data:
	let coin1 = Coin::Penny;
	let coin2 = Coin::Quarter(String::from("New York"));
	let coin3 = Coin::Dime;
	let coin4 = Coin::Nickel;

    println!("Penny value: {} cents", value_in_cents(coin1));  // Lucky penny! Penny value: 1 cents
	println!("Quarter value: {} cents", value_in_cents(coin2));  // State quarter from New York! Quarter value: 25 cents
	println!("Dime value: {} cents", value_in_cents(coin3));  // Dime value: 10 cents
	println!("Nickel value: {} cents", value_in_cents(coin4));  // Nickel value: 5 cents


	// Match with Gaurds
	let msg = Message::Move { x: 5, y: 5 };
    match_with_guard(msg);  // Moving to a diagonal coordinate: x = 5, y = 5
}


// Here is an example where enums store different types of data:
#[warn(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),  // This variant has an associated String (e.g., state for US quarters)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {}!", state);
            25
        }
    }
}


// Combine match with Gaurds to add additional consitions
fn match_with_guard(msg: Message) {
    match msg {
        Message::Move { x, y } if x == y => {
            println!("Moving to a diagonal coordinate: x = {}, y = {}", x, y);
        }
        Message::Move { x, y } => {
            println!("Moving to coordinates: x = {}, y = {}", x, y);
        }
        _ => println!("Other message"),
    }
}
