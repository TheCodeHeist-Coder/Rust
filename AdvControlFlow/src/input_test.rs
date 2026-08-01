use std::io;

pub fn test_input(){

    let name: &mut String = &mut String::new();

    println!("Enter you age: ");

    io::stdin().read_line( name).unwrap();

  let age:u8 = name.trim().parse::<u8>().unwrap();

  println!("The entered age is: {age}");

    
}