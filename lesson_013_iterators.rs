pub fn test_iterators() {
    let numbers = [1, 2, 3];
    let num_iter = numbers.iter();

    // Primitive
    // for num in num_iter {
    //     println!("Number {num}");
    // }

    let fruit_list = vec!["Apple", "Strawberry", "Guava"];
    let nut_list = vec!["Coconut", "Cashew", "Avocado"];
    let fruit_iter = fruit_list.iter();

    // let first_item = fruit_iter.next().unwrap();
    // let second_item = fruit_iter.next().unwrap();

    let aggregate_list = fruit_iter.chain(&nut_list);

    let all_foods: Vec<&&str> = aggregate_list.clone().collect();

    // println!("First item: {first_item}");
    // println!("Second item: {second_item}");

    // for food in aggregate_list {
    //     println!("Eating {food}...")
    // }

    let fruit_list_strings = fruit_list.iter().map(|e: &&str| String::from(*e));
    let new_fruits = fruit_list_strings.map(|mut e| {
        e.push_str(" food");
        return e;
    });

    // new_fruits
    //     .clone()
    //     .for_each(|fruit| println!("Eating {fruit}"));

    // new_fruits.clone().last();

    let first_names = vec!["Xela", "Tasha", "Bimbo"];
    let first_names_strings = first_names.iter().map(|e| String::from(*e));
    let last_names = vec!["Ola", "Sasha", "Osun"];
    let last_names_strings = last_names.iter().map(|e| String::from(*e));

    let full_names = first_names_strings.zip(last_names_strings);

    // full_names
    //     .clone()
    //     .for_each(|e| println!("Names are: {} {}", e.0, e.1));

    // Using enumerate
    // for (index, value) in full_names.clone().enumerate() {
    //     println!("{}th name is {} {}", index, value.0, value.1)
    // }

    let foods: Vec<(&str, u32)> = vec![("potatoes", 10), ("oranges", 20), ("apples", 30)];
    let food_total = foods.iter().fold(0u32, |acc: u32, x| acc + x.1);

    println!("Total foods are: {food_total}");
}
