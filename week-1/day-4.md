# Fuction

In Rust, a **function** is a small block of code that you can run whenever you need it to perform a specific task. Functions help you organize code so you can reuse tasks instead of rewriting the same code each time.

for video example visit https://www.youtube.com/watch?v=YnwiRiXu5gA&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=15

Here’s a basic way to understand it:

Imagine you want to make a cup of tea. The steps are the same each time:

1. Boil water
2. Add tea bag
3. Pour water into a cup
4. Let it steep

In Rust, you could create a function called `make_tea` that holds all these steps. Then, instead of explaining every single step each time, you can just say, "Run the `make_tea` function."

#### NOTE
The main fuction is the fuction that all functions are call inside it is where rust start execution from it specail type of fuction in rust

Here's what this looks like in Rust code:

```rust
fn make_tea() {
    println!("Boil water");
    println!("Add tea bag");
    println!("Pour water into a cup");
    println!("Let it steep");
}
```

With this function in place, anytime you need to make tea, you just call `make_tea()` instead of rewriting each step.
