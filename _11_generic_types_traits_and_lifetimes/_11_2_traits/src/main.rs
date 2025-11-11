/*
    Rules for Traits
        - Defining a Trait: A trait defines a set of methods that a type must implement.
            Some methods can have default implementations, while others must be implemented by the type.
        - Implementing a Trait: Types implement traits using the impl keyword followed by the trait and the type.
            Once a type implements a trait, it can use the methods defined by the trait.
        - Trait Bounds: Traits can be used as constraints for generic types, allowing you
            to specify that a type must implement a certain trait for it to be used in a function or struct.
        - Multiple Traits: A type can implement multiple traits, allowing it to use multiple behaviors.
        - Trait Objects: Traits can be used as types, allowing for dynamic dispatch (runtime polymorphism)
            when you don’t know the exact type at compile time.
        - Orphan Rule: In Rust, you can only implement a trait for a type if either the trait
            or the type is defined in the current crate (module).
            This prevents conflicts when multiple external crates are involved.        
*/

// Defining and Implementing a Trait
trait Describable {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}



// Default Implementation
trait Describables {
    fn describes(&self) -> String {
        String::from("An entity with no description.")
    }
}
#[allow(dead_code)]
struct Persons {
    name: String,
    age: u8,
}

impl Describables for Persons {}

fn main() {

    // // Defining and Implementing a Trait
    // let person = Person {
    //     name: String::from("Alice"),
    //     age: 30,
    // };
    
    // println!("{}", person.describe());


    // // Default Implementation
    // let person = Persons {
    //     name: String::from("Alice"),
    //     age: 30,
    // };
    
    // println!("{}", person.describes());  // Outputs the default description


    // // Trait Bounds and Generics
    // let num = 42;
    // let text = String::from("Hello, Rust!");
    
    // print_anything(num);
    // print_anything(text);


    // // Trait Objects and Dynamic Dispatch
    // let dog = Dog;
    // let cat = Cat;
    
    // animal_sound(&dog);
    // animal_sound(&cat);


    // // Multiple Trait Bounds
    // let numbers = Numbers {
    //     values: vec![1, 2, 3, 4],
    // };
    
    // print_and_sum(numbers);


    // Trait Implementations for Build-in Types
    let num: f32 = 5.5;
    println!("Double of {} is {}", num, num.double_float());
}


// Trait Bounds and Generics
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("i32: {}", self);
    }
}

impl Printable for String {
    fn print(&self) {
        println!("String: {}", self);
    }
}

fn print_anything<T: Printable>(item: T) {
    item.print();
}



// Trait Objects and Dynamic Dispatch
trait Animal {
    fn sound(&self) -> &str;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) -> &str {
        "Bark"
    }
}

impl Animal for Cat {
    fn sound(&self) -> &str {
        "Meow"
    }
}

fn animal_sound(animal: &dyn Animal) {
    println!("Animal sound: {}", animal.sound());
}


// Multiple Trait Bounds
trait Summable {
    fn sum(&self) -> i32;
}

trait Printables {
    fn print(&self);
}

struct Numbers {
    values: Vec<i32>,
}


impl Summable for Numbers {
    fn sum(&self) -> i32 {
        self.values.iter().sum()
    }
}

impl Printables for Numbers {
    fn print(&self) {
        println!("{:?}", self.values);
    }
}

fn print_and_sum<T: Printables + Summable>(item: T) {
    item.print();
    println!("Sum: {}", item.sum());
}


// Trait Implementations for Build-in Types
trait Double {
    fn double(&self) -> i32;
    fn double_float(&self) -> f32;
}

impl Double for i32 {
    fn double(&self) -> i32 {
        *self * 2
    }

    fn double_float(&self) -> f32 {
        *self as f32
    }
}

impl Double for f32 {
    fn double(&self) -> i32 {
        *self as i32
    }
    fn double_float(&self) -> f32 {
        *self as f32 * 2.0
    }
}