fn main() {
    // Call the `plus_five` function with an argument of 5 and assign the result to `result`.
    let result = plus_five(5);
    println!("Five plus one is: {result}");

    // Call the `display_blood_group` function with a blood group and a sign.
    display_blood_group('B', '+');
}

// Define a function `display_blood_group` that takes a blood group and a sign as parameters.
fn display_blood_group(blood_group: char, suffix: char) {
    println!("Your blood group is {blood_group}{suffix}");
}

// Define a function `plus_five` that takes an integer parameter `number` and returns an integer.
// This function adds 1 to `number` and returns the result.
// Note: The function uses an expression to return the result directly without a semicolon.
// The value of the expression is implicitly returned.
fn plus_five(number: i32) -> i32 {
    number + 1
}
