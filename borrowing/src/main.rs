

// fn main() {

//     let s: String = String::from("Hello");

//     let lenght:usize = calculate_length(&s);

//     println!("The len of {s} is {lenght}");
   

// }

// fn calculate_length(s:&String) -> usize {
//     s.len()
// }


//! Another example
//? Mutable references
// fn main(){

//     let mut s: String = String::from("Hello");

//     let len:usize = calculate_length(&mut s);

//     println!("The lenght of {s} is {len}");

// }

// fn calculate_length(s: &mut String) -> usize {
//     s.push_str("Raj");
//     s.len()
// }


//! Dangling references
