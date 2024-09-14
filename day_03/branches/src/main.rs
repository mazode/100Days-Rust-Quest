fn main() {
    let condition = true;

    /*  Use an `if` expression to assign a value to `number` based on the condition.
    If `condition` is true, `number` is assigned 4; otherwise, it's assigned 2. */
    let number = if condition { 4 } else { 2 };
    println!("Value of number is {number}");

    let a = 10;

    if a < 25 {
        println!("True");
    } else if a < 15 {
        println!("True");
    } else {
        println!("False");
    }
}
