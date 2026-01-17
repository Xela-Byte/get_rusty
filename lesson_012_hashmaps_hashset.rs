use std::collections::{HashMap, HashSet};

pub fn test_hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Black"), 30);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_score}");

    for (key, value) in &scores {
        println!("Team {key} has {value} scores!");
    }

    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{text_map:?}");

    let mut planets = HashSet::from(["Mercury", "Venus", "Earth"]);
    let more_planets = HashSet::from(["Jupiter", "Saturn", "Earth"]);

    planets.insert("Saturn");
    planets.insert("Pluto");
    planets.insert("Saturn");

    let planets_diff = planets.difference(&more_planets);
    let planets_sym_diff = planets.symmetric_difference(&more_planets);

    println!("Planets: {planets:?}");
    println!("Difference: {planets_diff:?}");
    println!("Symmetric Difference: {planets_sym_diff:?}");
}
