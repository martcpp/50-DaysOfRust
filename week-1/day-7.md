### 1. **Ownership**
Think of ownership like having the key to a car. If you have the key, you can drive it. In Rust, **ownership** means that only one person (or variable) has the key to use a piece of data at any time.

- If you give someone else the key, you can't drive the car anymore.
- When you're done with the car (your data goes out of scope), the car gets returned (the memory is freed).

videos here https://www.youtube.com/watch?v=8M0QfLUDaaA
more videos here https://www.youtube.com/watch?v=lQ7XF-6HYGc&t=1421s

Example:
```rust
let car1 = "Tesla";  // car1 owns the Tesla
let car2 = car1;     // car2 now owns the Tesla, car1 can't use it anymore
// car1 is no longer valid here.
```

So, when you pass ownership, only one person can use the car at a time.

### 2. **Borrowing**
Borrowing is like asking for the key to the car but without taking ownership of it. You’re just borrowing it for a while, and the owner still keeps the right to use the car after you're done.

There are two kinds of borrowing:

- **Immutable Borrowing** (no changes): You can borrow the car, but you can’t make any changes to it.
- **Mutable Borrowing** (you can change it): You can borrow the car, but now you’re allowed to change the radio station or adjust the seats, for example.

**The rules:**
- You can have many people (references) borrow the car if no one is allowed to change anything.
- Only one person can borrow it if they’re allowed to change something.

Example:
```rust
let car = "Tesla";

// Borrow without changing (immutable borrowing)
let read_borrow = &car;  // You can read it but not modify
println!("{}", read_borrow); // Ok!

// Borrow and change (mutable borrowing)
let mut car2 = String::from("Tesla");
let modify_borrow = &mut car2;  // You can change the car
modify_borrow.push_str(" Model S");  // You added to the car name
```

So, borrowing lets you use someone else's car (data) without taking it away from them.

### 3. **Lifetimes**
Lifetimes are like making sure the car (data) is still available while you’re using it. Rust ensures that you don’t hold onto a car key (a reference) after the car has been scrapped (data has been freed).

Imagine you borrow a car, but then the car gets sold or destroyed while you still have the key—now you’re stuck with a key to a car that no longer exists! Rust’s **lifetimes** prevent that situation by making sure you can only use the car as long as it exists.

Example:
```rust
fn which_is_longer<'a>(car1: &'a str, car2: &'a str) -> &'a str {
    if car1.len() > car2.len() {
        car1
    } else {
        car2
    }
}
```

Here, the `<'a>` makes sure both borrowed cars (data) are available as long as they’re being compared. It prevents you from trying to use a reference to something that’s already gone.

### Putting it Together
- **Ownership**: Only one person can own the car (data) at a time. When they’re done, the car is returned (memory freed).
- **Borrowing**: You can let others use the car (data) temporarily without giving them ownership. If they can change it, only one person can borrow it at a time.
- **Lifetimes**: Rust ensures that borrowed data is still around while you’re using it, so you never try to drive a car that doesn’t exist anymore.


