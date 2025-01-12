# Rust Mutable vs Immutable References

This repository demonstrates a common error in Rust programming related to mutable and immutable references and how to address the issue.

The `bug.rs` file contains code that attempts to modify a value through an immutable reference, resulting in a compile-time error. The `bugSolution.rs` file shows the correct way to handle mutable and immutable references to avoid such errors.

The code example highlights the importance of understanding Rust's borrowing rules to prevent data races and ensure memory safety.