### Reading list to learn Rust while building a single threaded HTTP server.

1. Difference between string and string slice in Rust:
``` rust
let string = String::from("127.0.0.1:8080");
let string_slice = &string[10..];
println!("{}", string_slice);
```

2. Ownership, Borrowing, References:
3 ownership rules in Rust:
a. Each value in Rust is owned by a variable.
b. When the owner goes out of scope, the value will be deallocated.
c. There can only be ONE owner at a time.
Borrowing:
a. Can ownership be borrowed? 

3. How does unwrap() work in Rust?
4. How does Heap and Stack allocation work in Rust?

