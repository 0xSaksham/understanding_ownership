# Rust Memory Management and Ownership

This Rust program is a comprehensive exploration of memory management and ownership in Rust. It covers various aspects, including cloning methods, the Copy trait, function scoping, return values, tuples, and mutable references. The code provides practical examples to showcase Rust's distinctive features in memory handling and ownership control.

## Cloning Methods

The `clone_method` function demonstrates the usage of the `clone` method to create a deep copy of a string.

```rust
fn clone_method() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

## Copy Trait

The `stack_copy_trait` function illustrates the Copy trait, which is applicable to types using the stack or having a known size at compile time.

```rust
fn stack_copy_trait() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

## Function Scoping

The program demonstrates function scoping with examples of taking ownership and making copies.

```rust
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}
```

## Return Values and Scope

The `gives_ownership` and `takes_and_gives_back` functions showcase Rust's ownership principles with return values.

```rust
fn gives_ownership() -> String {
    let some_str = String::from("Aloo");

    some_str
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

## Not Giving Ownership - Tuples and References

Two approaches to avoid giving ownership are presented - using tuples and references.

```rust
// Not giving Ownership Part-1(Tuples)

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}
```

## Mutable References

The `change` function demonstrates the use of mutable references to modify a string in-place.

```rust
fn change(some_str: &mut String) {
    some_str.push_str(" Shree Ram");
}
```

Explore the code to gain insights into Rust's powerful memory management and ownership model. Happy coding in Rust!
