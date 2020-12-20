/*
* primitive types for rust
*/
#[allow(unused_variables)]
fn main() {
    let logical: bool = true;
    let points: f64 = 93.32;

    // inferring types
    let mut _multi_logical = false;

    println!("{0}, {1}",logical,points);
    println!("{0}, {1}, {2}", logical, points, _multi_logical);
    println!("{0} {1} {2}", 1+2, 34i32 - 3, 2);

    println!("Booleans ops! {0} AND {1} is {2}",true, false, true&& false)
}