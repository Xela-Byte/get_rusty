use std::cell::Cell;

struct Person<'p> {
    first_name: Cell<&'p str>,
    last_name: String,
}

enum VehicleColor {
    Silver,
    Black,
    Gold,
    Titanium,
}

impl ToString for VehicleColor {
    fn to_string(&self) -> String {
        match self {
            VehicleColor::Black => String::from("Black"),
            VehicleColor::Silver => String::from("Silver"),
            VehicleColor::Gold => String::from("Gold"),
            VehicleColor::Titanium => String::from("Titanium"),
        }
    }
}

struct Vehicle {
    model: String,
    manufacturer: String,
    year: u16,
    color: VehicleColor,
}

struct VehicleTuple(String, String, u16, VehicleColor);

pub fn new_vehicle() -> Vehicle {
    let vehicle = Vehicle {
        model: String::from("M4"),
        manufacturer: String::from("BMW"),
        year: 2004,
        color: VehicleColor::Black,
    };

    return vehicle;
}

pub fn new_person() -> Person<'static> {
    let person = Person {
        first_name: Cell::from("Xela"),
        last_name: String::from("Ola"),
    };
    person.first_name.set("Sharryx");
    // person.first_name = Cell::from("Sharryx");
    return person;
}

pub fn test_create_person() {
    let person = new_person();
    println!("{}", person.first_name.get());
}

pub fn test_create_vehicle() {
    let vehicle = new_vehicle();
    println!("{}", vehicle.color.to_string())
}

pub fn new_vehicle_tuple() -> VehicleTuple {
    let vehicle_tuple = VehicleTuple(
        String::from("Hyundai"),
        String::from("x12"),
        2004,
        VehicleColor::Black,
    );

    return vehicle_tuple;
}

pub fn test_create_vehicle_tuple() {
    let vehicle_tuple = new_vehicle_tuple();
    println!("{}", vehicle_tuple.0)
}
