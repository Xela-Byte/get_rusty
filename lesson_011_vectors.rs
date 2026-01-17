enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn test_vectors() {
    let mut new_vector: Vec<u32> = Vec::new();
    let mut another_vector = vec![1, 2, 3];

    // Modifying a vector
    new_vector.push(1);
    new_vector.push(2);
    new_vector.push(3);

    let first_item = &another_vector[0];
    println!("First item: {first_item}");

    let second_item = another_vector.get(1).unwrap();
    println!("Second item: {second_item}");

    // Will panic
    // let does_not_exist = &another_vector[100];

    //  Will return None
    let does_not_exist_2 = another_vector.get(100);

    for i in &another_vector {
        println!("Listing items: {i}");
    }

    // Mutating
    for i in &mut another_vector {
        *i *= 2
    }

    println!("{another_vector:?}");

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
