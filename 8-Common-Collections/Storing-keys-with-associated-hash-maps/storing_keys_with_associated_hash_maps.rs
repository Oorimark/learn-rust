use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let blue_key = String::from("Blue");

    // getting blue score
    let score = scores.get(&blue_key).copied().unwrap_or(0);
    println!("The blue score is {score}");

    // printing keys, and value
    for (key, value) in &scores {
        println!("{key} => {value}");
    }

    // checking entry then inserting
    scores.entry(String::from("Yellow")).or_insert(60);
    scores.entry(String::from("Red")).or_insert(20);
    println!("{:?}", scores);

    // incrementing a value of hash map
    let words = String::from("hello world wonderfull world");
    let mut map = HashMap::new();

    for word in words.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
