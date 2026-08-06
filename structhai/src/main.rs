pub mod example_basic;
pub mod implbasic;
fn main() {
    println!("Hello, world!");

    let person: example_basic::Person = example_basic::new_person();
    println!("Person is: {}",person.first_name);

    example_basic::create_vehicle();

    example_basic::create_vehicle_tuple();

    implbasic::create_vehicle_new();
} 
