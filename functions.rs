/*
* function implementation for rust
*/

// A rust function should have input and return types explicitly 
// given
// a statement without ; will be considered return statement
fn add(a:i32, b:i32) -> i32 {
    a+b
}

fn main() {
    println!("This is a main function, core for running a rust program");
    println!("{}",add(3,4))
}