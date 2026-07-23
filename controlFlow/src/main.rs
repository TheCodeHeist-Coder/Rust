
fn main() {
    println!("Hello, world!");

    
    // let x: i32 = 10;
    // if x == 5 {
    //     println!("This is true");
    // } else {
    //     println!("This is false")
    // }

    
    // let ans: () = if true {println!("Yes i'm in")} else {println!("No, i'm out")};
    
    //? Loops

    //? example - 01
    // loop {
    //     println!("Hii, I'm raj!");
    // }

    //? example - 02
    // let mut counter: i32 = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);

    //? example - 03
    //? Here this counting_up is the lebel for the break statement
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");

    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");

    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");


    //? while loop
    // let mut number = 5;

    // while number != 0 {
    //     println!("Number is: {number}");
    //     number -= 1;
    // }

    // println!("Exited");


    //? another while loop example
    // let a: [i32; 5] = [10, 20, 30, 40, 50];

    // let mut index = 0;

    // while index < 4 {
    //     println!("The value is: {}", a[index]);
    //     index += 1;
    // }


    //? For loop
    // let a:[i32; 5] = [1, 2, 3, 4, 5];

    // for element in a {
    //     println!("The value is: {element}");
    // }


    //? another for loop example
    for number in (1..5).rev() {
        println!("Number is: {number}");
    }

    println!("Exited!!!")

}
