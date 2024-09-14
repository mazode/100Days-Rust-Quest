use std::io;

fn main() {
    // Get the conversion option from the user
    let option = get_conversion_option();

    // Get the temperature from the user
    let temp = get_temperature();

    // Perform the conversion based on the user's option and display the result
    match option {
        1 => println!("{temp}°F is equal to {}°C", fahrenheit_to_celsius(temp)),
        2 => println!("{temp}°C is equal to {}°F", celsius_to_fahrenheit(temp)),
        _ => println!("Invalid option. Please restart the program and select 1 or 2."),
    }
}

fn get_conversion_option() -> u32 {
    // Prompt the user to select the conversion type
    println!("Convert to:\n1- Celsius\n2- Fahrenheit");

    // Read the user's selection
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read option");

    // Parse the user selection to a u32
    option
        .trim()
        .parse()
        .expect("Please enter a valid option (1 or 2)")
}

fn get_temperature() -> f64 {
    println!("Enter the temperature: ");

    // String to read the temperature input
    let mut temp = String::new();

    // Read the temperature input
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature");

    // Parse the temperature input
    temp.trim()
        .parse()
        .expect("Please enter a valid temperature")
}

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * 9.0 / 5.0) + 32.0
}
