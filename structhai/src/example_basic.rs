
//? Example - 01
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub birth_year: u16,
    pub birth_month: u8,
}

pub fn new_person() -> Person {
    let p1: Person = Person {
        first_name: "Raj".to_string(),
        last_name: "Kumar".to_string(),
        birth_year: 2005,
        birth_month: 7,
    };

    p1
}



//? Example 02

#[derive(Debug)]
enum VehicleColor {
    Silver,
    Blue,
    Green,
    Black,
    White
}

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor
}


fn new_vehicle() -> Vehicle {
    let v1 = Vehicle {
        manufacturer: "Tata".to_string(),
        model: "XFF1".to_string(),
        year: 2003,
        color: VehicleColor::Silver
    };
    return v1;
}

pub fn create_vehicle(){
    let my_vehicle = new_vehicle();
    println!("Vehicle is: {:?}", my_vehicle)

}



//? Example 03
#[derive(Debug)]
struct VehicleTuple(String, String, u16);

fn new_vehicle_tuple() -> VehicleTuple {
    return VehicleTuple("Raj".to_string(), "Syndra".to_string(), 2020);
}

pub fn create_vehicle_tuple() {
    println!("{:?}", new_vehicle_tuple());
}




