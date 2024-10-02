# variables in Rust
for video explain vist https://www.youtube.com/watch?v=J3fv1-1SgI4&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=4


In Rust, a variable is like a container that holds some kind of value, and you can give it a name so you can easily refer to it later. However, unlike some other languages, Rust variables are **immutable** by default, which means once you put something in that "box," you can't change it unless you specifically say so.

Here's a simple example in Rust:

```rust
fn main() {
    let age = 25; // Here, we're creating a variable called 'age' and storing the value 25 in it.
    println!("Your age is: {}", age); // We can use the variable by referring to its name.
}
```

In this example:
- `let` is used to declare a variable in Rust.
- `age` is the name of the variable.
- `25` is the value we stored in the variable `age`.

Now, if you want to change the value in the "box" (make it mutable), you would need to use `mut` to make the variable **mutable**:

```rust
fn main() {
    let mut age = 25; // The 'mut' keyword allows the value to be changed later.
    println!("Your age is: {}", age);

    age = 26; // Now we can change the value.
    println!("Next year, you'll be: {}", age);
}
```

In this case, we:
- Declared `age` as mutable (`mut`).
- Changed its value from `25` to `26` later in the program.

So, in Rust:
- Variables are **immutable** by default, which helps prevent accidental changes.
- You need to use `mut` if you want to change the value of a variable.


send a PR to ADD SOMETHING
