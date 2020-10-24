/*
* function for rust tuples.
* rust tuples are datastructure which can store any type of data types.
*/

// A function to reverse the (i32, bool) pair
fn reverse(pair:(i32, bool))-> (bool, i32) {
    let (interger, boolean) = pair;
    (boolean, interger)
}

fn main() {
    let entry_tuple = (9u32,8u32);
    println!("{0}, {1}", entry_tuple.0, entry_tuple.1);
    
    // using tuples as tuple members
    let tuple_in_tuple = ((23u32, 23u32),(23i32, 2u32,), (false, true));
    println!("{:?}", tuple_in_tuple);

    // This is causing error in displaying
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("{}", too_long_tuple)

    let reversed_pair = (32i32, true);
    println!("{:?}",reverse(reversed_pair));

    // Deconstructing tuples to primitives
    let pat = (23,23,32);
    let(a,b,c) = pat;
    println!("{0} {1} {2}", a, b, c)
}