// This program shows off the basics of loops in Rust
// By: Nick from CoffeeBeforeArch

fn main() {
    // We can have infinite loops that require a break to exit from
    let mut id = 0;
    loop {
        // Here we use a condition for our break
        if id == 10 {
            break;
        }

        // Otherwise, keep incrementing
        id += 1;
    }
    println!("The value of id is {}", id);

    // We can do the same thing with a while loop
    // Here we count back down to 0
    while id != 0 {
        id -= 1;
    }
    println!("The value of id is {}", id);

    // What about traversing an array?
    // We can use a for loop (similar to python, or range-based ones in C++)
    let array = [1, 2, 3, 4, 5];
    for element in array.iter() {
        println!("Element from array: {}", element);
    }

    // We can also use ranges in our loops (very similar to Python)
    // NB: It's non-inclusive (1..6 gives us numbers from 1 to 5)
    for number in 1..6 {
        println!("Number from range: {}", number);
    }
}
