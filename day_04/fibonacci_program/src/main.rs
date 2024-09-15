use std::io;

fn main() {
    // Get the input
    println!("Enter the number: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    // Convert Input String to Integer type
    let input: i32 = input.trim().parse().expect("Enter a valid number");

    //Call the function to computer fibonacci number
    let result = fibonacci_sequence(input);
    println!("The {input}th Fibonacci number is: {result}");
}

// Function to compute the nth Fibonacci number
fn fibonacci_sequence(input: i32) -> i32 {
    // Handle the base cases
    if input == 0 {
        return 0;
    } else if input == 1 {
        return 1;
    }

    let mut a = 0; // Previous Fibonacci Number (n - 2)
    let mut b = 1; // Current Fibonacci Number (n - 1)

    // Iterate from 2 to the user `input` number to compute the Fibonacci sequence
    for _ in 2..=input {
        let temp = b; // Temporarily store the current Fibonacci number
        b = a + b; // Compute the new Fibonacci number
        a = temp; // Update the previous Fibonacci number
    }

    // Return the current Fibonacci number
    b
}
