// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

// Hint
//
// Vectors in rust make use of generics to create dynamically sized arrays of any type.
// You need to tell the compiler what type we are pushing onto this vector.

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
