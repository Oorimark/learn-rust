fn main (){
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(5);
    v1.push(3);
    v1.push(2);
    v1.push(1);
    v1.push(8);
    v1.push(9);
    v1.push(7);

    // Reading elements of vectors
    let third: &i32 = &v2[2]; // using indexing

    // using the get method
    let third: Option<&i32> = v2.get(2);    
    if let Some(third) = third {
        println!("The Third element is {}", third);
    } else { println!("Theres' no third element")};

    /* Iterating a Vector */

    // Iterating a vectors with immutable reference
    for i in &v1 { println!("{}", i) }

    // Iterating a vector with mutable reference
    for i in &mut v1 { println!("{}", *i += 50) }

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];

    
}
