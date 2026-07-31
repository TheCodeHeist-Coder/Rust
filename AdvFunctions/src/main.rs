pub mod helpers;

fn main() {
    println!("Hello, world!");
    let name_result: String = helpers::namehelpers::get_full_name("Raj", "Kumar");
    println!("The full name is: {}", name_result);
}






// #[allow(dead_code)]
// fn print_name(){
//     println!("Hello! Raj");
// }