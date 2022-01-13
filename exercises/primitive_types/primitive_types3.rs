// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// Hint
//
// There's a shorthand to initialize Arrays with a certain size that does not
// require you to type in 100 items (but you certainly can if you want!).
// For example, you can do:
// let array = ["Are we there yet?"; 10];

// Bonus: what are some other things you could have that would return true
// for `a.len() >= 100`?


fn main() {
    let a = ["Hi"; 200];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
