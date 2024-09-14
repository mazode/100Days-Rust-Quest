// Array Iteration
fn array_iteration() {
    let array = [1, 2, 3, 4];
    for element in array {
        println!("Value = {element}");
    }
}

// While Loop
fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF");
}

// Labeled Loop with Nested Loops
fn labeled_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn main() {
    // Call each function to execute the code snippets.
    array_iteration();
    while_loop();
    labeled_loop();
}
