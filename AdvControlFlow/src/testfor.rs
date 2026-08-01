
pub mod testing_for_loop {
    
//! Basic concepts
pub fn basic_for_concepts() {

    //? This will print from 1 to 9
    for i in 1..10 {
        println!("{i}");
    }

    //? if we want to include the last one then
    for i in 1..=10 {
        println!("{i}");
    }

    //? looping over an array
    let demo_names: [&str; 4] = ["Raj", "Shoes", "Looo", "Laalo"];
    for (idx, name) in demo_names.iter().enumerate() {
        println!(" {idx}: {name} ");

    }

    //? iterating over Strings
    let word = "Hello";
    for c in word.chars() {
        println!("{c}");
    }

   




}

}