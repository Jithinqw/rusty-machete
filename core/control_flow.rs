pub enum FlowTypes {
    IFELSE,
    LOOP,
    WHILE,
    FOR,
    RANGE,
    MATCH,
}

fn call_control_flow(e:FlowTypes) {
    match e {
        FlowTypes::IFELSE => if_else(23),
        FlowTypes::LOOP => loop_fn(323),
        FlowTypes::WHILE => while_fn(),
        FlowTypes::FOR => for_in_fn(),
        FlowTypes::RANGE => for_in_iter(),
        FlowTypes::MATCH => for_in_iter()
    }
}

fn if_else(age:i32) {
    if age > 18 {
        println!("You are eligible for voting")
    } else if age < 15 {
        println!("You are a child")
    } else {
        println!("You are not eligible for voting")
    }
}

fn loop_fn(loop_age: i32) {
    loop {
        if loop_age == 3 {
            break
        }
        if loop_age > 5 {
            continue
        }
    }
}

fn while_fn() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", n)
        }

        // Increment counter
        n += 1;
    }
}

fn for_in_fn() {
    for n in 0..3223 {
        if n % 13 == 0 {
            println!("13 is a dangerous number")
        } else {
            break
        }
    }
}

fn for_in_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Bob" => println!("Hey from bob"),
            &"Frank" => println!("Hey from Frank"),
            &"Ferris" => println!("Hey from Ferris"),
            _ => println!("Hey from nobody")
        }
    }
}

fn main() {
    call_control_flow(FlowTypes::WHILE)
}
