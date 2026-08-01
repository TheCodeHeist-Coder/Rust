pub fn test_closures() {
    // let add = || println!("Returning some text");
    // add();

    // let add = |x:i8, y:i16| println!("Some text is here which is {x} and also {y}");
    // add(4, 10);

    // let add = |x:i16, y:i16| x + y;
    // add(4, 10);


    //? Programme
    let add = |x: i8, y: i8| {
        println!("x is: {x} and y is: {y}");
        x + y
    };
    let result = add(4, 10);
    
    let print_result = || println!("The result is: {result}");
    print_result();
}


/****
 //! Notes
    A closure is an anonymous function that can capture variables from its surrounding environment — something regular fn functions cannot do. Think of it as a "function value" you can store in a variable, pass around, and call later.

    //! Basic syntax
    //? let closure_name = |parameters| expression;
 * 
 */