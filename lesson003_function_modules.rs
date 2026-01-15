// Public modules
pub mod name_helpers {
    // public module function
    pub fn get_full_name(first_name: &str, last_name: &str) -> String {
        let full_name: String = format!("{0} {1}", first_name, last_name);

        // returning full name
        full_name
    }

    // private module function
    fn get_last_name(last_name: &str) -> String {
        String::from(last_name)
    }
}

// private function

fn test_function() {
    println!("Greetings from private functions");
}
