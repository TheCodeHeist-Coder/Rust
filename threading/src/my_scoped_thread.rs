

struct Person {
    first_name: String
}

pub fn test_thread_variables() {
    let age: i32 = 34;

    let person01= Person{
        first_name:String::from("Rajkumar")
    };

    let print_age = || {
        println!("Your age is: {age}");
        println!("Your name is: {}", &person01.first_name);
    };

    //? scope() thread
    std::thread::scope(|scope| {
        scope.spawn(print_age);
    });

    //? Normal thread
    // let _result = std::thread::spawn(print_age).join();


    println!("Your age is: {age}");
    println!("Your name is: {}", person01.first_name);

    println!("FInished printing age");
}