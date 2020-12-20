/*
* Rust as a language does not have a garbage collector,
* so it is a programmers duty to manage memory.
* Rust does it by using a concept called ownership and borrowing.
* 
* Rust have two memory management model, Stack and heap.
* A stack memory model is the model Rust uses by default 
* it is used when declaring all the primitive values,
* it is extremly memory efficient and faster. 
* The stack is used when Rust knows if, what amount of 
* memory is allocated at compile time.
* Examples of this model is i32, u32, bool, Array etc....
* Why is this memory model is used beccause it is 
* extremly efficeint, it can copy/ clone the memory location
* faster or in efficient time.
*
*
*
* A heap momory model is used when Rust does not have an
* idea of how much memory is needed in compile time
* or the variable grows in time.
* Exmaples are String, Vec, Box, etc..
* For this memeory model it is not very efficient to 
* clone a memory location since the memory is either not known or 
* it can contain millions of items.
*
* There are some exceptions to this model, such as Struct, 
* Which can use either Stack / Heap memory model according to 
* the usage.
*/

fn main() {
    let primitive_i32:i32  = 34; // these uses a stack memory model
    let array_st: [i32;5] = [0;5];
    println!("{}", primitive_i32);
    for item in array_st.iter() {
        println!("{}", item);
    }

    let ownership_string: String = String::from("hello"); // this uses a heap model
    println!("{}", ownership_string);
    analyze_model(&ownership_string); 
    // here & symbol is used a borrowing system, which give the control from 
    // the main variable to the function, and when the scope of 
    // that function ends, the ownership is given back to the 
    // main variable (owership_string).
    println!("{}", ownership_string)
}

#[allow(unused_variables)]
fn analyze_model(param: &String) -> i32{
    10
}