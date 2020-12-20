/*
* rust array example
* Array is a collection of same types T
* let mut array_temp = [T;size]
* an array is not iterable, you have to convert it as iterable using .iter() 
* we need to add size attached to its signature.
*/


use std::mem;

#[allow(unused_variables)]
fn main() {
    // fixed size array
    let xs: [i32; 3] = [23,3,23];
    for item in xs.iter() {
        println!("{}",item);
    }

    // or you can just iter over the cloned array and pass it.
    for item in &xs {
        println!("{}", item);
    }

    println!("{}", xs.len());

    println!("My simple array is taking {} much memeory", mem::size_of_val(&xs));

    // we can make intitilize all element
    let init_array: [i32;5] = [0;5];
    for items in &init_array {
        println!("{}", items)
    }
    
}