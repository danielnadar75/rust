#[allow(unused_mut, unused_variables)]
use std::collections::HashMap;

fn main() {
    // 1. Creating HashMap
    let mut scores = HashMap::new();

    // 2. Setting Values
    scores.insert(String::from("Blue"), 90);
    scores.insert(String::from("Red"), 70);
    println!("Scores: {:?}", scores);

    // 3. Getting Values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue teams's score is {score}");

    // 4. Iterating Over HashMap
    for (key, value) in &scores {
        println!("{key} -> {value}");
    }

    // 5. HashMap & Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name}"); // ERROR -> variable moved!
    /* ERROR Info: field_name and field_value are invalid at this point */

    // 6. Overwriting valures
    scores.insert(String::from("Green"), 10);
    scores.insert(String::from("Green"), 25);
    println!("{:?}", scores);

    // 7. Conditional Insert (Avoid Overwriting)
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(100);
    println!("{:?}", scores);

    // 8. Update values based on the only values
    let mut text = "Hello world wonderful world";
    let mut words = HashMap::new();

    for word in text.split_whitespace() {
        let mut count = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Words: ${:?}", words);
}
