use std::io;

fn main() {
    // Define a static integer
    let integer_type: i32 = 48;
    println!("Static integer: {integer_type}");

    // Define a static floating-point number
    let float_type: f64 = 2.52354;
    println!("Static floating point number: {float_type}");

    // Define a static boolean
    let boolean_type: bool = true;
    println!("Your Bool Type is: {boolean_type}");

    // Define a static character
    let character_type: char = 'A';
    println!("The character is: {character_type}");

    // Define a `tuple` with an integer, a floating-point number, and an unsigned integer
    let tup: (i32, f64, u8) = (500, 2.3, 1);

    // Access and store the elements of the tuple
    let index_zero = tup.0;
    let index_one = tup.1;
    let index_two = tup.2;

    // Define an array of integers
    let array = [1, 2, 3, 4];
    println!("Enter the array index:");

    // Create a mutable String variable to store user input
    let mut index = String::new();

    // Read user input
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // Convert the input string to a usize for array indexing
    let index: usize = index
        .trim()
        .parse()
        .expect("Indexed entered  was not a number");

    // Access and store the element at the specified index in the array
    let element = array[index];
    println!("The element at index {index} in the array is: {element}");
}
