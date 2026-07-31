
pub mod if_else_concepts {
   pub fn test_if() {
    let age_to_drive: u8 = 16u8;

    println!("Enter the person's age: ");
    let myinput: &mut String = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();


    let age: u8 = myinput.replace("\n", "").parse::<u8>().unwrap();

    if age >= age_to_drive {
        println!("Issuing driver's license...");
    }
    else {
        println!("Cann't be Issued...");
    }


    //? bool
    let driver_license = if age >= 16 {true} else {false};
    println!("{driver_license}");

}
}