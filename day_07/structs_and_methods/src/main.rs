// Struct definition for a User
struct User {
    name: String,
    age: u32,
    active: bool,
    email: String,
}

// Implement methods for the User struct
impl User {
    // Associated function to create a new user
    fn new(name: String, age: u32, email: String) -> User {
        User {
            name,
            age,
            active: true, // default value
            email,
        }
    }

    // Method to print user details
    fn print_details(&self) {
        println!(
            "Name: {}, Age: {}, Email: {}, Active: {}",
            self.name, self.age, self.email, self.active
        );
    }

    // Method to update the email
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
        println!("Email updated to: {}", self.email);
    }
}

// Function to demonstrate struct usage
fn main() {
    // Create a new user using the associated function
    let mut user1 = User::new(String::from("Alice"), 30, String::from("alice@example.com"));

    // Print user details
    user1.print_details();

    // Update email
    user1.update_email(String::from("alice@newmail.com"));

    // Print updated details
    user1.print_details();
}
