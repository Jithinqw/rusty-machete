///
/// Closures are like Python lambda functions
///

fn main() {
    /// Closure using ||
    let perper = |i:i32| -> i32 {i+1};
    println!("{}", perper(23));
    /// Closure no parameters
    let oneortwo = || 1;
    println!("{}", oneortwo())
}
