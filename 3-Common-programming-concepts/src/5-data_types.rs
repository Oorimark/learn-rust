fn main(){
    /* SCALER TYPES */

    // Signed Integers
    let x: i8 = -3;
    let x: i32 = -3;
    let x: i64 = -3;
    let x: i128 = -3;
    let x: isize = -3;

    // Unsigned Integers
    let x: u8 = 3;
    let x: u32 = 3;
    let x: u64 = 3;
    let x: u128 = 3;
    let x: usize = 3;

    // Floating f64 and f32 
    // NB: It only supports signed floating point
    let x: f32 = 24.2;
    let x: f64 = 24.2;

    // Boolean Type
    let x: bool = true;
    let x: bool = false;

    // Character Type
    let c: char = 'z';

    /* COMPOUND TYPES */

    // Tuples
    let tup: (u8, f32, i8) = (3, 3.23, -3);

    let (x, y, z) = tup;
    println!("The value of y is {y}");
}
