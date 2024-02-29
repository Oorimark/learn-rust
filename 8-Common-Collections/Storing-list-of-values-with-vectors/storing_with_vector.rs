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

    let third: Option<&i32> = v2.get(2);    
    // using the get method
    if let Some(third) = third {
        println!("The Third element is {}", third);
    } else { println!("Theres' no third element")};
}
