pub mod helpers;

/******
 //! All about this above line
  //? What This Means
mod helpers; (note the semicolon, no curly braces) tells the Rust compiler: "There's a module named helpers — go find its contents in another file."
pub makes that module public, so its public items are accessible from outside this file/parent module.


 */


fn main() {
    println!("Hello! This is the advanced functions class!");

    let name: String = helpers::test_module::get_full_name("Raj", "Kumar");
    //? Here, we are using this helpers::something, because we're using the function which are in helper file and in test_module named module
    println!("The full name is: {name}");
}

//? making unused function
#[allow(dead_code)]

/** 
 * These are called attrubutes in Rust
 * Attributes in Rust are written with #[...] and are processed by the compiler to change how it behaves for that item.
dead_code is a compiler lint. Rust's compiler proactively warns about code that is never used — for example, functions that are defined but never called anywhere in the program. This helps catch bugs, leftover debug code, or unfinished work.
allow(...) tells the compiler: "suppress this particular warning for this item." So #[allow(dead_code)] explicitly says: "Yes, I know this function might not be called anywhere — don't warn me about it."


//! Why would you use this?
//?Common real-world reasons:

The function is part of a library's public API, intended for external use, but nothing in this crate calls it directly.
You're in the middle of development and haven't wired up the function yet, but don't want to be flooded with warnings.
The function exists for future use, debugging, or testing, and you don't want the "unused" noise cluttering your build output.

  */

fn unused_function() {
    println!("This is unused unused_function");
}
