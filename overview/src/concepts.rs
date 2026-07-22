

//! Important Slides Link
//?  https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-1






// fn main(){
//! Variables
// let x: i32 = -2;  // here i32 is a signed integer of 32 bits (+ or -)
// let y: u32 = 20;  // u32 is an unsigned integer
// let z: f32 = 12.32; // number can be + , - or decimal

// print!(
//     "x: {}, y: {}, z: {}", x, y, z
// )

//? ___________OR_________

// print!("x: {}", x);
// print!("y: {}", y);
// print!("z: {}", z);

//! booleans
// let is_male = true;
// let is_above_18 = true;

// if is_male {
//     println!("You are a male");
// } else {
//     println!("You are not a male")
// }

// if is_male && is_above_18 {
//     println!("You are good to go!")
// }

//! strings
// let name: String = String::from("Hello! Raj");
// println!("{}", name);

//? print the first letter
// Se'll later

//! Mutability
//  let x:i8 = 10;
//  x = x + 2;  //? This will give an error

// let mut x:i8 = 12;   //? it'll work because we add mut
// x = x + 2;
// println!("{}", x)

//! shadowing

//? Now, here first varibal x is shadowed by the second x veriable
// let x:i16 = 20;
// print!("{}", x);

// let x:i16 = 30;

// print!("{}", x);

//! Compound data types ---> Tuples and Arrays

//!  Tuples ---> Can have multiple types
// let tup: (i32, u16, f64) = (10, 28, 20.3);

//? Accessing an element
// let x = tup.0;
// let y = tup.1;
// let z = tup.2;
// print!("{x}, {y}, {z}");
//?_________ OR ___________

// let tup = (500,12.8, 30);
// let (x, y, z) = tup;     //? pattern matching by desctructing a tuple

// print!("The value of x, y, z is: {}, {}, {}", x, y, z);

//! Array type --> only have same typs of elements

// let a: [i32; 5] = [1, 2, 3, 4, 5];  //? Declaration of an array
// let a = [2; 5];   //? This will give [2, 2, 2, 2, 2]

// //? Accessing an element
// let a:[i32; 5] = [1, 2, 3, 4, 5];

// let x = a[0];
// let y = a[3];

// print!(" {x} ");
// print!(" {y} ");

//! Conditionals and loops
// let is_even: bool = true;

// if is_even {
//     println!("Number is even");
// }else {
//      {
//         println!("Number is not even");
//      }
// }

//? for loop
// for i in 0..10 {
//     print!("{} ", i);
// }

//! Funtions...
//? function declaration
//  println!("Hii, this is the main function");
//  test(10,'h');

//? statements and expressions
//  statements_expression();

//? return value in funtions
//    let x: i32 = return_val();
//    println!("The return value is: {}", x);

// let x:i32 = return_val(10);
// println!("The return value is: {}", x);

//! Using if in let statement
// let you: bool = true;
// let x = if you {10} else {20};
// println!("The value of the number is: {}",x);

//! loop, for and while

//? loop
//  loop {            //? run over and over again infinitly untill we stop it manually
//      println!("Again");
//  }

// let mut counter = 0;
// let result = loop {
//    counter += 1;

//    if counter == 10 {
//     break counter * 2;
//    }
// };

// println!("The value of counter is: {}",result);

//! Ownership   ---> Some set of rules for memory management

//? Example 01
// let s1 = String::from("Hello");
// let s2 = s1;

// println!("{}", s1);  //? Give an compilation error

// }

//? funtion declaration
// fn test(x:i32, unit:char){
//     println!("Hii, This is the test funtion in {x}{unit}");
// }

//? statements and expressions
// fn statements_expression(){
// let x = 6;  //? this is the statement, doesn't return any kind of value

//     let y = {
//         let x = 5;
//          x + 10  //? Here, we didnt use semicolon, because it's a expression ending and returning something(x + 10), but if we use ;, then it'll not return anything
//     };

//     println!("The value of y is: {}", y);
// }

//? return value in functions
// fn return_val() -> i32 {
//     10
// }

// fn return_val(x:i32) -> i32 {
//     x + 20
// }

//? Example 02 of ownership
// fn main(){

//     //! Ownership
//     //? Example 02

//     let my_string = String::from("Hello");   //? now my_string is the owner of this heap data
//     takes_ownership(my_string);     //? now the owner of the data "Hello"  is the some_string

//     println!("{}", my_string);       //? So now this will give an compile error because ownership has been moved

// }

// fn takes_ownership(some_string:String){
//     println!("{}",some_string);           //? some_string now owns the data
// }

//?  __________________How can it work ??________________

//     //! Ownership
// fn main(){

//     let mut my_string = String::from("Hello");
//    my_string = takes_ownership(my_string);

//     println!("{}", my_string);

// }

// fn takes_ownership(some_string:String) -> String{
//     println!("{}",some_string);
//     return some_string
// }

//! References
// fn main(){
//     let s1 = String::from("Hello");

//     let s2 = &s1;

//     println!("{}", s2);
//     println!("{}", s1);  //? This is valid, The first pointer wasn't invalidated
// }

//! Borrowing
// fn main(){
//     let my_string = String::from("Hello");
//     borrow_variable(&my_string);  //? pass a reference to my_string
//     println!("{}", my_string);        //? This is a valid because ownership was not transferred

// }

// fn borrow_variable(some_string: &String){
//     println!("{}", some_string); //? Some_string is borrowed and not moved
// }

//! Mutable references --> If you want a function to `update` the value of a variable
// fn main(){
//     let mut s1 = String::from("Hello");
//     upadate_word(&mut s1);
//     println!("{}", s1);
// }

// fn upadate_word(word: &mut String){
//     word.push_str(" World");
// }

//! Structs
//? Structs in rust let you structure data together. Similar to Objects in JS

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sing_in_count: u64
// }

// fn main(){

//     let user1 = User {
//         active: true,
//         username: String::from("rajk123"),
//         email: String::from("raj@google.com"),
//         sing_in_count: 1
//     };

//     print!("User 1 username: {:?}", user1.username);

// }

//! implementing structs

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn parimeter(&self) -> u32 {
//         2 * (self.width + self.height)
//     }
// }

// fn main(){
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     print!("The area of the rectangle is {}", rect.area());
//     print!("The area of the rectangle is {}", rect.parimeter());
// }

//! Enums
//? Similar to typeScript. They allow you to define a type by enumerating its possible varients

// enum Direction {
//     North,
//     East,
//     South,
//     West
// }

// fn main(){
//     let my_direction: Direction = Direction::North;
//     move_around(my_direction);

// }

// fn move_around(direction: Direction){
//     // your logic here
// }

// //! Pattern matching and enums with values
// //? It let's you pattern match across various varients of an enum and run some logic

// //? Define an enum calles Shape
// enum Shape{
//     Circle(f64),      // varients with associated data (redius)
//     Squere(f64),      // varients with associated data (side length)
//     Rectangle(f64, f64) // varients with associated data (width, height)
// }

// //? Funtion to calculate area based on shapes
// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
//         Shape::Squere(side_length) => side_length * side_length,
//         Shape::Rectangle(widht, height, ) => widht * height,
//     }
// }

// fn main(){
//     //? Create instances of various shapes
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Squere(4.0);
//     let rectangle = Shape::Rectangle(3.0, 5.0);

//     //? Calculate and print the areas
//     println!("Area of circle: {}", calculate_area(circle));
//     println!("Area of rectangle: {}", calculate_area(rectangle));
//     println!("Area of square: {}", calculate_area(square));

// }



//! Error Handling
//? By using the Result Enum

// use std::fs;

// //? Result enum gives us two things, which we can see below
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn main() {
    // there is a fn that can error out/stop the thread
//     let res = fs::read_to_string("example.txt");

//     match res {
//         Ok(content) => {
//             println!("File Content: {}", content);
//         }

//         Err(err) => {
//             println!("Error: {}", err);
//         }
//     }
// }



//! Option enum
//? The option enum was introduced in Rust to handle the concept of nullability
//? in a safe and expressive way.Unlike many programming languages that use a null or similar keyword to represent the absence of a value, Rust doesn't have null

// pub enum Option<T> {
//     None,
//     Some(T)
// }

//? If you ever have a function that should return null, return an Option instead
// fn find_first_a(s:String) -> Option<i32> {
//     for(index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn main(){
//     let my_string = String::from("Rajkumar");
//     match find_first_a(my_string) {
//         Some(index) => println!("The letter 'a' is found at index: {}", index),
//         None => println!("The letter 'a' is not found in the string"),
        
//     }
// }



//! These `Result` and `Option` are the most commanly used enums in 

