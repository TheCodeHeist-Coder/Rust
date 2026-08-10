

fn main() {
    println!("Hello, world!");

    // test_vec_int();

    // test_vec_string();

    test_vec_car();
    
}


//? Declaring the vector
fn test_vec_int() {

    let mut  my_ints: Vec<i32> = Vec::new();

    my_ints.push(12);
    my_ints.push(17);
    my_ints.push(11);
    my_ints.push(1);
    my_ints.push(200);

   println!("Size of the vector: {:?}", my_ints.len());
   println!("Capacity of the vector: {:?}", my_ints.capacity()); 

    println!("{:?}", my_ints);

    // println!("Firts item is: {:?}", &(&my_ints).as_slice()[0..]);

    //? if not element then return None rather than panic
    println!("The first element is: {:?}", my_ints.get(3));
}


fn test_vec_string() {

   let names: Vec<&str> = vec!["Raj", "Loos", "laao", "Matin"];

   //? we can also use clone()
    for name in names.as_slice() {
        println!("Processing {} ..." , name);
    }


    println!("{:?}", names);
}


#[derive(Debug)]
#[allow(dead_code)]
struct Car {
    name: String,
    model: String

}

fn test_vec_car() {
    let mut car_list: Vec<Car> = vec![];

    let mut car_lot2: Vec<Car> = vec![];

    for _ in 1..=5_000_000_u32 {
        car_list.push(Car{name:"Tootr".to_string(), model:"2020".to_string()});
    }

     for _ in 1..=100u8 {
        car_lot2.push(Car{name:"Hyundai".to_string(), model:"2020".to_string()});
    }

    car_list.append(&mut car_lot2);


    car_list.insert(0, Car{name:"Toyato".to_string(), model:"@030".to_string()});
    car_list.remove(0);

    let keep = |e: &Car|  if e.name == "Tootr" {return true;} else {return false;};
     car_list.retain(keep);


     car_list.reserve(5000);

    println!("{:?}", car_list);
    
    println!("{:?}", car_list.len());
    println!("{:?}", car_list.capacity());

    println!("{:?}",car_lot2);


    println!("{:?}", car_list.get(0));

    let mut input: String = "".to_string();
    std::io::stdin().read_line(&mut input).expect("Something bugggy here...");
}