pub fn test_types() {
    // Unsigned integers
    let a: u8 = 1;
    let b: u16 = 2;
    let c: u32 = 3;
    let d: u64 = 4;
    let e: u128 = 5;
    let f: usize = 6;

    // Signed integers
    let g: i8 = -1;
    let h: i16 = -2;
    let i: i32 = -3;
    let j: i64 = -4;
    let k: i128 = -5;
    let l: isize = -6;

    // Floating integers
    let n: f32 = 2.0;
    let o: f64 = 3.0;

    // Struct
    struct Person {
        first_name: String,
        age: u16,
        cgpa: f32,
    }

    // String
    let name: String = String::from("value");
    // String
    let emoji: &str = "🥷🏽";

    // Tuple
    let our_dictionary: (i32, i32, i32, String) = (1, 2, 3, String::from("Xela"));

    // Array
    let our_fruits: [&str; 3] = ["Mango", "Strawberry", "Coconut"];

    // Vectors
    let our_books: Vec<&str> = vec!["Chemistry", "physics", "biology"];

    // boolean
    let is_true: bool = true;
    let is_false: bool = false;
}

pub fn test_variables() {
    // mutable variable
    let mut x: u8 = 5;
    x = 6;

    let mychar: char = '🔥';

    // testing range
    let our_fruits: [&str; 3] = ["Mango", "Strawberry", "Coconut"];
    let new_fruits = &our_fruits[0..=2];
    println!("{:?}", new_fruits);
}
