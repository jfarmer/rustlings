// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// Hint
//
// This time, the function *declaration* is okay, but there's something wrong
// with the place where we're calling the function.

fn main() {
    call_me(20);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
