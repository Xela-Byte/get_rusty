#[derive(Debug)]
enum VehicleColor {
    Black,
    White,
    Titanium,
}

#[derive(Debug)]
struct Vehicle {
    model: String,
    manufacturer: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, color: VehicleColor) {
        self.color = color
    }

    fn create_vehicle() -> Self {
        Self {
            model: String::from("Avendor"),
            color: VehicleColor::Black,
            manufacturer: String::from("Lamboghini"),
            year: 2004,
        }
    }
}

pub fn test_create_vehicle() {
    let vehicle_result = Vehicle::create_vehicle();
    println!("{vehicle_result:?}");
}
// pub fn create_vehicle() {
//     let mut vehicle = Vehicle {
//         model: String::from("Avendor"),
//         color: VehicleColor::Black,
//         manufacturer: String::from("Lamboghini"),
//         year: 2004,
//     };

//     vehicle.paint(VehicleColor::Titanium);

//     println!("{:?}", vehicle)
// }
