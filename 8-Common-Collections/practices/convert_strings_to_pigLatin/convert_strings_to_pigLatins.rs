/*
* Exercise:
* Convert strings to pig latin. The first consonant of each word is
* moved to the end of the word and “ay” is added, so “first” becomes
* “irst-fay.” Words that start with a vowel have “hay” added to the
* end instead (“apple” becomes “apple-hay”). Keep in mind the details
* about UTF-8 encoding!
*
*/

const VOWELS_ASCII_DEC: [u8; 10] = [65, 69, 73, 79, 85, 97, 101, 105, 111, 117];

fn main() {
    let mut new_sentence: String = String::new();
    let sentence = String::from("Hello World How are you today");

    // loop through the words
    for word in sentence.split_whitespace() {
        let new_word: String;
        let mut first_char: Option<char> = None;

        // collecting the first character in the word
        if let Some(fc) = word.chars().next() {
            first_char = Some(fc);
        };

        if let Some(fc) = first_char {
            let first_char_bytes = fc as u8;

            if VOWELS_ASCII_DEC.contains(&first_char_bytes) {
                new_word = format!("{word}-hay");
            } else {
                let popped_word = word[1..].to_string();
                new_word = format!("{popped_word}{fc}ay");
            }
            new_sentence += &format!(" {new_word}");
        }
    }

    // output
    println!("{sentence}");
    println!("{new_sentence}");
}
