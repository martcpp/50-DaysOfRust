### Conditionals (Making Decisions)
Conditionals let the program choose what to do based on some condition (whether something is true or false).

vidoe for conditionals can be fine here https://www.youtube.com/watch?v=mLizOfB1bZE&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=6

vidoe for loop can be fine here https://www.youtube.com/watch?v=t76yT6sNtzQ&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=8

**Example:** Let's say you want to check if someone is old enough to vote.

```
fn main() {
    let age = 18;

    if age >= 18 {
        println!("You can vote!");
    } else {
        println!("You are too young to vote.");
    }
}
```

Here’s what’s happening:
- `if age >= 18`: If the `age` is 18 or more, the program prints "You can vote!".
- `else`: If the condition isn't true (meaning `age` is less than 18), it prints "You are too young to vote."

Rust also allows `else if` for multiple conditions:

```
fn main() {
    let temperature = 25;

    if temperature > 30 {
        println!("It's hot outside!");
    } else if temperature < 10 {
        println!("It's cold outside!");
    } else {
        println!("The weather is nice.");
    }
}
```

### Loops (Repeating Actions)
Loops allow you to repeat an action multiple times.

1. **`loop`**: This runs forever until you tell it to stop.

   ```
   fn main() {
       let mut count = 0;

       loop {
           count += 1;
           println!("Count is: {}", count);

           if count == 5 {
               break; // stop the loop when count is 5
           }
       }
   }
   ```

2. **`while`**: This runs as long as a condition is true.

   ```
   fn main() {
       let mut count = 0;

       while count < 5 {
           count += 1;
           println!("Count is: {}", count);
       }
   }
   ```

   In this example, the loop stops when `count` reaches 5.

3. **`for`**: This loop is used to go through a range of values or a collection like a list.

   ```
   fn main() {
       for number in 1..5 {
           println!("Number: {}", number);
       }
   }
   ```

   Here, `for` goes through each number from 1 to 4 (`1..5` means 1 up to, but not including, 5).

### Summary
- **Conditionals** (`if`, `else`, `else if`): Help the program make decisions based on conditions.
- **Loops** (`loop`, `while`, `for`): Help repeat actions multiple times based on certain conditions or ranges.
