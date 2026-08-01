pub fn match_now() {


    //? test on intergers
    let my_age: u16 = 5;

    match my_age {

        0 | 5 => println!("Your age is 0"),
        20 => {
            println!("Your age is 35");
        }
        21..200 => {
            println!("Your age is between 21 and 200")
        }

        200 => {
            println!("Your age is 200");
        }

        1..=19 => {
            println!("Your age is upto 18");
        }

        200.. => println!("Your age is over 200"),


       
    }

}


pub fn test_match_string() -> u32 {

    let name: &str = "Raj";

    //? first usecase
    // match name {
    //     "kumar" => println!("Your name is kumar"),
    //     "Raj" => println!("Your name is in list"),
    //     _ => println!("Name is not here in this programme")
    // }

    //? Second usecase
    match name {
        "Kumar" => 2000,
        "Raj" => 5000,
         _ => 0
    }

}


pub fn test_match_array() {

    let prices: [u32; 3] = [7000, 12000, 12000];

    match prices[0..=1]{

        [5000, 6000] => println!("You are right now"),
        [7000, 12000, ..] => println!("Anything else is remained"),
        _ => println!("You have nothing")
        
    }

}
