/*
* Rust does not provide implict type casting between primitive types.
* But the types can be converted by as keyword
*/

fn main() {
    let primitive_t: u8 = 3;
    println!("{}", primitive_t as char)
}