pub fn test_generics() {
    fn find_largest(number_list: &[i32]) -> &i32 {
        let mut largest = &number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = find_largest(&number_list);

    println!("The largest number is {largest_number}");
}

trait Animal {
    fn make_sound(&self) -> ();
}

#[derive(Debug)]
struct Person<PetType: Animal> {
    first_name: String,
    pet: PetType,
}

#[derive(Debug)]
struct Dog {}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog barked!");
    }
}

#[derive(Debug)]
struct Cat {}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Cat meowed!");
    }
}

#[derive(Debug)]
struct Bear {}
impl Animal for Bear {
    fn make_sound(&self) -> () {
        println!("Bear roared!");
    }
}

#[derive(Debug)]
struct Tiger {
    stripes: u32,
}
impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Tiger roared!");
    }
}

pub fn test_create_person() {
    let pet_result = Tiger { stripes: 20 };
    pet_result.make_sound();
    let person_result = Person {
        first_name: "Xela".to_string(),
        pet: pet_result,
    };

    println!("{:?}", person_result);
}
