// This program shows off the basics of function calls in Rust
// By: Nick from CoffeeBeforeArch

// Our own first function (no params, no return)
// Rust doesn't care where they're defined!
fn print_func() {
    println!("This is a print from a function!");
}

// Here's a function that takes two integers and prints them out
fn arg_func(x: i32, y: i32) {
    println!("Value of x is {}, and value of y is {}", x, y);
}

// Here's one that take an int, and returns an ent
fn return_func(x: i32) -> i32 {
    // NB: Return statements don't end in semicolons!
    // That's because statements don't return a value
    x + 1
}

fn main() {
    // Simple function (no params, no return)
    print_func();

    // Function with params (no return)
    arg_func(10, 37);

    // Function with param and return
    let x = return_func(10);
    println!("The returned value of x is {}", x);
}

