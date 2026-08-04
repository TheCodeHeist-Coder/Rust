pub mod example_basic;

fn main() {
    println!("Hello, world!");

    let person: example_basic::Person = example_basic::new_person();
    println!("Person is: {}",person.first_name);

    example_basic::create_vehicle();
} 
