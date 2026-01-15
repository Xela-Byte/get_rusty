pub mod test_closures {

    struct Person {
        first_name: String,
        last_name: String,
    }

    pub fn closure_function() {
        let add = |x: u32, y| println!("this is a closure, {}", x + y);
        add(5, 4);

        let mut person = Person {
            first_name: String::from("Xela"),
            last_name: String::from("Ola"),
        };
        //
        let mut change_name = |new_last_name: &str| person.last_name = String::from(new_last_name);
        change_name("Xeloao");

        println!(" last name is: {}", person.last_name)
    }
}
