fn main() {
    let mut s1 = String::from("foo");

    // pushing a string
    s1.push_str("bar");
    println!("Result for s1 is {}", s1);

    // pushing a character
    s1.push('.');

    let s2 = String::from("Hello");
    let sentence = s2 + "World";
    println!("I'm saying {}", sentence);

    // printing characters of a string
    println!("\nPrinting chars of {}", sentence);
    for c in sentence.chars() {
        println!("{c}")
    }

    // printing bytes of a string
    println!("\nPrinting bytes of abcdef");
    for b in "abcdef".bytes() {
        println!("{b}")
    }
}
