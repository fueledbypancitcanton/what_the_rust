fn main() {
    
    // Output (Print Text)
    println!("Hello, world!");
    
    // This is a comment
    /* Integers
    i8, i16, i32, i64, i128, */
    let x: i32 = -2;
    println!("This is an intger: {}", x);

    /* Floats
    f32
    f64
    */
    let floating_point: f64 = 10.92;
    println!("This is a float: {}", floating_point);

    // Boolean
    let true_or_false: bool = true;
    println!("This is a boolean: {}", true_or_false);

    // Char
    let letter: char = 'a';
    println!("This is a character: {}", letter);

    // Tuples
    let mut tup:(i32, bool, char) = (1, true, 's');
    println!("{}", tup.0);
    tup = (5, false, 'b');
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    // Arrays
    let mut arr = [1,2,3,4,5];
    arr[4] = 10;
    println!("{}", arr[4]);
}
