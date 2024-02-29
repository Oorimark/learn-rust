fn main(){
    let s: String = String::from("Hello");

    let len = calculate_len(&mut s);
    println!("The length of string {} is {}", s, len);
}

fn calculate_len(s: &mut String) -> usize {
    s.push_str("World");
    s.len()
}
