pub mod test_control_flow {

    pub fn test_if() {
        let age_to_drive: u8 = 16u8;
        let age_input: &mut String = &mut String::from("");
        println!("Please input your age: ");
        std::io::stdin()
            .read_line(age_input)
            .expect("Invalid input of some sort.");

        // one method of type casting
        // let parsed_age = age_input.replace("\n", "").parse::<u8>().unwrap();

        // my preferred method
        let parsed_age: u8 = age_input.replace("\n", "").parse().unwrap();

        if parsed_age < age_to_drive {
            println!("Ogbeni, you no fit drive yet.");
        } else if parsed_age == age_to_drive {
            println!("Be like say you just sabi.")
        } else {
            println!("Get in jor!")
        }
    }

    pub fn test_while() {
        let age: u8 = 16u8;
        let mut current_age: u8 = 0u8;

        while current_age < age {
            println!("Waiting for eligibility...");
            current_age += 1;
            if current_age == age {
                println!("Congratulations, you're now eligible to drive at {current_age}!");
            } else {
                println!("Current age: {current_age}");
            }
        }
    }

    pub fn test_loop() {
        let age: u8 = 16u8;
        let mut current_age: u8 = 0u8;

        loop {
            if current_age == age {
                println!("Congratulations, you're now eligible to drive at {current_age}!");
                break;
            } else {
                println!("Waiting for eligibility...");
                println!("Current age: {current_age}");
            }

            current_age += 1;
        }
    }

    pub fn test_for() {
        let age_to_drive: u8 = 16u8;
        let ages: [u8; 5] = [14, 16, 18, 20, 22];

        for age in ages {
            if age >= age_to_drive {
                println!("Congratulations, you're now eligible to drive at {age}!");
            } else {
                println!("Waiting for eligibility...");
                println!("Current age: {age}");
            }
        }
    }
}
