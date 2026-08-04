pub mod option_type;

fn main() {
    println!("Hello, world!");

    let result = option_type::test_option_type();
    println!("{}", result.unwrap());

    let result = option_type::test_option_type_string();
    println!("{}", result.unwrap());


    let charcter = option_type::test_option_type_chartype();
    println!("{}", charcter.unwrap().to_string());
}
