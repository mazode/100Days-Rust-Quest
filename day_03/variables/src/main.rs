fn main() {
    // Declare a mutable variable `x` and initialize it with 3
    let mut x = 3;
    // Shadow `x` with the value `x + 1`
    let x = x + 1;
    {
        // Shadow the outer scope `x` with `x * 2` in the inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}");

    // Shadow `spaces` to hold its length
    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
