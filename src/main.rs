


fn main(){
    //! Variables
    // let x: i32 = -2;  // here i32 is a signed integer of 32 bits (+ or -)
    // let y: u32 = 20;  // u32 is an unsigned integer
    // let z: f32 = 12.32; // number can be + , - or decimal

    // print!(
    //     "x: {}, y: {}, z: {}", x, y, z
    // )

    //? ___________OR_________

    // print!("x: {}", x);
    // print!("y: {}", y);
    // print!("z: {}", z);


    //! booleans
        // let is_male = true;
        // let is_above_18 = true;

        // if is_male {
        //     println!("You are a male");
        // } else {
        //     println!("You are not a male")
        // }

        // if is_male && is_above_18 {
        //     println!("You are good to go!")
        // }


    //! strings
        // let name: String = String::from("Hello! Raj");
        // println!("{}", name);

        //? print the first letter
         // Se'll later



    //! Mutability
    //  let x:i8 = 10;         
    //  x = x + 2;  //? This will give an error

    // let mut x:i8 = 12;   //? it'll work because we add mut
    // x = x + 2;
    // println!("{}", x)


    //! shadowing
     
       //? Now, here first varibal x is shadowed by the second x veriable
        // let x:i16 = 20;
        // print!("{}", x);

        // let x:i16 = 30;

        // print!("{}", x);


        //! Compound data types ---> Tuples and Arrays
        

        //!  Tuples ---> Can have multiple types
        // let tup: (i32, u16, f64) = (10, 28, 20.3);

        //? Accessing an element
        // let x = tup.0;
        // let y = tup.1;
        // let z = tup.2;
        // print!("{x}, {y}, {z}");
        //?_________ OR ___________
        
        // let tup = (500,12.8, 30);
        // let (x, y, z) = tup;     //? pattern matching by desctructing a tuple

        // print!("The value of x, y, z is: {}, {}, {}", x, y, z);

     //! Array type --> only have same typs of elements
    
        // let a: [i32; 5] = [1, 2, 3, 4, 5];  //? Declaration of an array
        // let a = [2; 5];   //? This will give [2, 2, 2, 2, 2]

        // //? Accessing an element
        // let a:[i32; 5] = [1, 2, 3, 4, 5];

        // let x = a[0];
        // let y = a[3];

        // print!(" {x} ");
        // print!(" {y} ");






        //! Conditionals and loops
            // let is_even: bool = true;

            // if is_even {
            //     println!("Number is even");
            // }else {
            //      {
            //         println!("Number is not even");
            //      }
            // }


         //? for loop
            // for i in 0..10 {
            //     print!("{} ", i);
            // }





}
