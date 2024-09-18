# Day 7

## Summary

Today, I completed Chapter 5 of the Rust book, which focused on **Structs** and **Method Syntax**.

### 1. Structs

- **Structs** allow you to group related data into a single entity.
- Structs are defined using `struct` and can hold multiple fields of different types.

### 2. Defining and Instantiating Structs

- You can define structs with named fields.
- Structs can be instantiated by providing values for the fields, like:
  `let user = User { name: String::from("Alice"), age: 30 };`

### 3. Associated Functions & Methods

- Associated functions can be created using `impl` blocks.
- You can define methods on structs using `self` as a parameter to operate on struct instances.

### Key Learning

- How to define and use structs effectively in Rust to model more complex data.
- How to implement methods for structs to encapsulate behavior.
