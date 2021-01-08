fn main() {
    let mut b2: Vec<i32> = vec![34,3434,34,34,];
    println!("{:?}", b2);
    for i in &b2 {
        println!("{}", i);
    }
    println!("{}", &b2.len());
    for i in 1..10 {
        b2.push(i);
    }
    for i in &b2 {
        println!("{}", i)
    }
}