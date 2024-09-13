# Day 2 Notes

## Guessing Game Project

### Key Concepts Learned:

- **Random Number Generation**: Used `rand::Rng` to generate a random number between 1 and 100.
- **Handling User Input**: Used `std::io::stdin()` to read input from the user and handled errors using `expect()`.
- **Looping**: Used a `loop` to repeatedly prompt the user for input until the correct guess is made.
- **Pattern Matching**: Implemented `match` to handle different comparison outcomes (`Less`, `Greater`, `Equal`) when comparing the user's guess to the secret number.
- **Type Conversion**: Converted a string to a number using `guess.trim().parse()`, handling errors with `match` to ensure the program doesn’t crash.

### Takeaways:

- Rust’s error handling (e.g., `expect`, `match`) allows the program to deal with unexpected input without crashing.
- Learned how to handle user input, type conversions, and control flow effectively.
