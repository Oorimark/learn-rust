
fn main (){
    let tree_attrs: [&str; 12] = [
        "And partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let ordinal_numbers: [&str; 12] = [
        "first", 
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelveth"
    ];

    for position in 0..12{
        let mut tree_attr_line = String::new();
        let first_line = format!("On the {} day of christmas", ordinal_numbers[position]);
        let second_line = "my true love gave to me";

        if position == 0 { 
            tree_attr_line = "A partridge in a pear tree,".to_string();
        } else {
            for attr_position in (0..position+1).rev(){
                tree_attr_line += &format!("{}{}", tree_attrs[attr_position], ",\n").to_string();
            }; 
        };

        // print lyrics
        println!("{},\n{},\n{}\n", first_line, second_line, tree_attr_line);
    }


}
