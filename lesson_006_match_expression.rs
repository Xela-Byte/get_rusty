pub fn test_match_int() {
    let my_age: u16 = 22;
    let y: u16 = 40;

    match my_age {
        // range type match
        1..22 => println!("You're up to or same age as me!"),
        100 => println!("You're foking old!"),
        22..40 if y == my_age => println!("A youth at 40!"),
        _ => println!("Too bad!"),
    }
}

pub fn test_match_string() {
    let car_manufacturer: &str = "porche";

    match car_manufacturer {
        "hyudai" => 3000,
        "toyota" => 3500,
        _ => 0,
    };
}

pub fn test_match_array() {
    let scores: [u16; 5] = [30, 14, 77, 65, 29];

    match scores[0..=2] {
        [14, 30] => println!("first scorer!"),
        _ => println!("Defaulter"),
    }
}
