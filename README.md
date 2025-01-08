# Subtle Data Race in Rust with Multiple Mutable References

This repository demonstrates a subtle data race bug in Rust.  The code compiles without warnings, yet exhibits undefined behavior due to multiple mutable references to the same value.

The `bug.rs` file shows the problematic code. The `bugSolution.rs` offers a solution by using appropriate synchronization mechanisms to ensure thread safety and prevent data races.  See the detailed explanation within the comments of each file.