// This program shows off the basics fo conditional control flow in Rust
// By: Nick from CoffeeBeforeArch

fn main() {
    // Condition tests must be a boolean! (can't convert from an int, etc)
    let test = 6;
    if test % 4 == 0 {
        println!("Test is divisible by 4!");
    } else if test % 3 == 0 {
        println!("Test is divisible by 3!");
    } else if test % 2 == 0 {
        println!("Test is divisible by 2!");
    } else {
        println!("Test is not divisible by 4, 3, or 2!");    
    }

    // "if" is an expression, it can be on the right side of a let statement!
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is {}", number);
}
