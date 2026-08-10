use std::ops::Index;

fn main() {
    println!("Hello, world!");

    test_rust_iterators();
}

fn test_rust_iterators() {
    let fruits_list: Vec<&str> = vec!["Banana", "Blue", "Mango", "Orange", "Apple"];

    let nut_list = vec!["Walnut", "Almonds", "Pecans", "Non"];

    let mut fruit_iter = fruits_list.iter();

    fruit_iter.next(); //? it'll point to Banana
    fruit_iter.next(); //? It'll point to Blue

    let item01 = fruit_iter.next();

    println!("First item in iterator is: {}", item01.unwrap());

    //? testing chain() method
    let aggregate_foods = fruits_list.iter().chain(&nut_list);


    let all_foods:Vec<&&str> = aggregate_foods.clone().collect();


    for food in aggregate_foods {
        println!("Eating {}", food);
    }


    //? converting &str into the String
    let fruit_list_strings = fruits_list.iter().map(|e| String::from(*e));

  let new_fruits = fruit_list_strings.map(|mut e| {e.push_str(" fruit"); return e;});

  new_fruits.clone().for_each(|e| println!("{}", e));



  //? grab last element
  let last_fruit = new_fruits.clone
  ().last().unwrap();

  println!("Last fruit is: {}", last_fruit);



  //? skip items
 let mut  step_by_elements = new_fruits.clone().step_by(2);
 println!("{}", step_by_elements.next().unwrap());
 println!("{}", step_by_elements.next().unwrap());



 //? zip()
 let first_name = vec!["Raj", "Yuvi", "Bhuvi", "Kane"];
 let first_name_strings = first_name.iter().map(|e| String::from(*e));

 let last_names = vec!["Kumar", "Singh", "Kumarr", "Willy"];
 let last_names_strings = last_names.iter().map(|e| String::from(*e));

 let full_names = first_name_strings.zip(last_names_strings);
//  full_names.for_each(|e: (String, String)| println!("{} {}",e.0, e.1));


//? Enumeration
// for (index, value) in full_names.enumerate() {
//     println!("Index: {0} value: {1} {2}", index, value.0, value.1);
// }
 

//? skiping items
//  full_names.skip(2).take(1).for_each(|e| println!("{}", e.0));



//? flod() it's a reduce()
let foods = vec![("potatos", 10), ("burgers", 20), ("pizza", 19)];
let food_quantity: u32 =  foods.clone().iter().fold(0u32, |mut acc,e: &(&str, u32) | acc + e.1 );

println!("Total quantity is: {}", food_quantity);


//? why this error
foods.iter().peekable().next();
 let food = foods.clone().iter().peekable().peek();
 println!("Peeking at: {}", food.unwrap().0);



}
