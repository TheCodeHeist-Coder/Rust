use crate::implbasic::VehicleColor::Gray;


#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColor {
    Gray,
    Blue,
    Green,
    Black,
    Gold
}

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor
}

impl Vehicle {
    //! Instance level methods
    fn paint(&mut self, new_color:VehicleColor) {
        self.color = new_color;
    }

    //* */ Static methods
    fn create_vehicle() -> Vehicle {
        let new_vehicle = Vehicle{
            manufacturer: "Default".to_string(),
            model: "202".to_string(),
            year: 2020,
            color: VehicleColor::Blue
        };
        new_vehicle
    }


}

fn new_vehicle() -> Vehicle {
    let mut v1 = Vehicle {
        manufacturer: "Tata".to_string(),
        model: "900".to_string(),
        year: 2024,
        color: VehicleColor::Gray
    };
    
    //* */ Calling instace method
    v1.paint(VehicleColor::Gold);
    v1
}

pub fn create_vehicle_new() {
    // let my_vehicle = new_vehicle();

    //! Calling static method
    let mut  my_vehicle = Vehicle::create_vehicle();
    my_vehicle.paint(VehicleColor::Black);
    println!("{:?}", my_vehicle);
}