pub mod test_module {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{first} {last}");
        full_name
    }
}

/****
 * 
 //! All about this pub mod some_thing{}
 Module Declaration — pub mod test_module { ... }
mod creates a module — a namespace to group related code (functions, structs, other modules, etc.).
pub makes the module public, so it's visible/accessible from outside the file or parent module it's declared in.
Everything inside the { } belongs to test_module. To call something inside it from elsewhere, you use the path syntax: test_module::get_full_name(...).

//? Note: Modules don't need to be in separate files — you can nest them directly inline like this, or split them into their own .rs files as a project grows. Inline modules like this are common for organizing code within a single file (e.g., grouping helper functions, or separating test code with #[cfg(test)] mod tests { ... }).
 
//! All about format!()
  format!("{first} {last}")
format! is a macro that builds and returns a String (works like println!, but returns the string instead of printing it).
{first} and {last} are captured identifiers — a feature since Rust 2021 edition that lets you reference variables directly by name inside the {} placeholders, instead of passing them as separate arguments.
Older/equivalent style: format!("{} {}", first, last)
Explicit positional style: format!("{0} {1}", first, last)
All three produce identical output — the captured-identifier style is just more concise and readable.
The literal " " (space) between {first} and {last} is inserted as-is.
let full_name = ...
No type annotation needed — the compiler infers full_name: String automatically, since format! always returns String.


**/
