use std::collections::{HashMap, HashSet};


 fn main() {
    println!("Hello, world!");

    // test_hashmap_basic();
    test_hashset_basic();
}


#[allow(dead_code)]
fn test_hashmap_basic() {

    //? Creating
    let mut stock_list: HashMap<String, f32> = HashMap::new();

    println!("{}",stock_list.is_empty());
     println!("{}",stock_list.len());


     //? Inserting
     stock_list.insert("Raj".to_string(), 500.30);
     stock_list.insert("Kumar".to_string(), 200.21);
     stock_list.insert("HHOL".to_string(), 302.98);


     //? update
     stock_list.insert("Kumar".to_string(), 999.99);

     stock_list.entry("META".to_string()).or_insert(100.00);
     
     println!("{:?}", stock_list);
      println!("{}",stock_list.is_empty());
     println!("{}",stock_list.len());


     //? Removing

     stock_list.remove(&("Raj".to_string()));

     println!("{:?}", stock_list);


     //? Iteration
     for (stock, value ) in stock_list {
        println!("{} is trading at {}", stock, value);
     }

}





fn test_hashset_basic() {
    
    let planets: HashSet<&str> = HashSet::from(["Earth", "Mars"]);

    //   for planet in planets {
    //     println!("Thanks for adding: {}", planet);
    // }

    let another_planets = HashSet::from(["Mai", "Tum", "And", "Earth"]);

    // normal difference like sets in mathematics
   let planet_diff =  planets.difference(&another_planets);

   //? similar removed
   let symmetric_diff = planets.symmetric_difference(&another_planets);

   for planet in symmetric_diff {
    println!("symmetric_diff is: {}", planet);
   }

   for planet in planet_diff {
    println!("Differ planets are: {}", planet);
   }
  


    


}