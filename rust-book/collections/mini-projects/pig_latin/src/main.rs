fn main() {
    println!("Hello, Collection's Mini-Project!");
    println!("02: The pig-latin!\n");

    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let sentance = "alison goldman".to_string();
    println!("[Original]: {sentance}");

    let words = sentance.split_whitespace();

    // for v in sentance.chars() {
    //     println!("{}", v);
    // }

    let mut result = String::new();

    for word in words {
        let first_letter = word.chars().next().unwrap_or(' ');
        // println!("first: {}", first_letter);

        if VOWELS.contains(&first_letter) {
            result.push_str(&format!("{word}-hay "));
        } else {
            result.push_str(&format!("{}-{first_letter}ay ", &word[1..]));
        }
    }

    println!("[Pig-Latin]: {}", result);
}
