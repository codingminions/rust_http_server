## Reading list to learn Rust while building a single threaded HTTP server.\

#### 03/25:\
1. **Difference between string and string slice in Rust:**\
``` rust
let string = String::from("127.0.0.1:8080");
let string_slice = &string[10..];
println!("{}", string_slice);
```\

2. **Ownership, Borrowing, References:**\
Three ownership rules in Rust:\
a. Each value in Rust is owned by a variable.\
b. When the owner goes out of scope, the value will be deallocated.\
c. There can only be ONE owner at a time.\
Borrowing:\
a. Can ownership be borrowed?\ 

3. How does unwrap() work in Rust?\
4. How does Heap and Stack allocation work in Rust?\

03/26:
1. How are structs, strings, enums and option enums defined and used in Rust?\
2. How do we use the **match** expression?\
3. What are traits in Rust, eg: TryFrom, From? How do we define and use them? Use cases?\
4. What is **Option**, **Result** in Rust? Converting an Option into a Result.\
5. What is **if let** expression?\
6. What is **lifetimes** in Rust?\

