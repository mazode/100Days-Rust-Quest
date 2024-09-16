fn main() {
    // Array storing gifts for each day
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    // Array storing the day names
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // Loop through each day of Christmas
    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            days[day]
        );

        // Loop backwards to print all the gifts for the current day
        for gift in (0..=day).rev() {
            if gift == 0 && day != 0 {
                // Add "and" before the first gift (except on the first day)
                print!("and ");
            }
            println!("{}", gifts[gift]);
        }
        println!(); // Add a blank line after each day
    }
}
