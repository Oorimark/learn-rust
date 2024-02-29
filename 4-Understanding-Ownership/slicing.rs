fn main(){
    let sentence = String::from("Hello World, I am learning Rust");
    let first_word = first_word(&sentence);

    println!("The first word is {}", first_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s[..];
}
