///
/// Structs can be 
/// C- Type
/// Tuple 
/// Unit
/// 

struct CustomError {
    error_type: String,
    error_code: i32,
    error_response: String
}

fn main () {
    let errorFn = CustomError{error_type:String::from("File Error"), error_code: 34, error_response:String::from("Something went wrong in handing file")};
    println!("{}", errorFn.error_response)
}