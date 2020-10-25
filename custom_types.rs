/*
* A custom types for Rust mainly 
* Struct, Tuple(Type of struct), Enums, constants
*/

struct PersonPoint {
    name: String,
    address: String
}

enum StatusType {
    Dead,
    Alive
}

enum Number {
    Zero,
    One,
    Two
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let person_address: PersonPoint = PersonPoint { name: "Jithin".to_string(), address: "Parkins st UK".to_string()};
    println!("{}", person_address.name);
    println!("{}", person_address.address);

    let status = StatusType::Dead;
    match status {
        Dead => println!("Dead are no longer available"),
        Alive => println!("We are all available")
    }

    // Enums can be used like C type enums
    // type casted to i32
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("Roses are {}", Color::Red as i32);

    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" })
}