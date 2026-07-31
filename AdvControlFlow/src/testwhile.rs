
pub mod testing_while_loop {
    pub fn test_while() {

    let age_to_drive: u8 = 18u8;
    let mut  current_age = 0u8;
    while current_age < age_to_drive {
        println!("Waiting!.. {current_age} ");
        current_age += 1;
    }
}

}