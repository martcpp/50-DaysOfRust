In Rust, data types tell the program what kind of data it is dealing with, much like how you use containers in real life to store specific items (e.g., water goes in a bottle, books go on a shelf).

for video examples see https://www.youtube.com/watch?v=o9mCVvEuKGg&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=5

some basic data types in Rust:

### 1. **Integers** (Whole Numbers)
- Think of integers as the type for counting or numbers without any fractions (like 1, 2, 100).
- Example: `let age: i32 = 25;`
  Here, `i32` is a type for 32-bit integers.

### 2. **Floats** (Decimal Numbers)
- Floats are numbers with decimal points, like 3.14 or 9.99.
- Example: `let price: f64 = 10.99;`
  `f64` means it's a floating-point number with double precision (64-bit).

### 3. **Booleans** (True/False)
- This is like a switch that can either be `true` (on) or `false` (off).
- Example: `let is_student: bool = true;`

### 4. **Characters** (Single Letter/Emoji)
- A `char` in Rust is for a single character, like a letter or even an emoji.
- Example: `let grade: char = 'A';` or `let smiley: char = '😊';`

### 5. **Strings** (Text)
- A `String` is like a list of characters strung together to form words or sentences.
- Example: `let name: String = String::from("Mart");`

### 6. **Tuples** (Mixed Data Group)
- Tuples let you group different types of values together, like a mini package that can hold multiple items.
- Example: `let person: (String, i32) = (String::from("Mart"), 25);`
  This holds both a name and an age.

### 7. **Arrays** (Fixed-size List)
- Arrays are a collection of items of the same type, like a row of identical storage boxes.
- Example: `let numbers: [i32; 3] = [1, 2, 3];`
  This is an array of three integers.

### 8. **Vectors** (Growable List)
- Vectors are like arrays, but they can grow and shrink in size.
- Example: `let mut scores: Vec<i32> = vec![85, 90, 78];`

These data types are used in Rust to ensure that the program knows how to handle different kinds of information efficiently. fine more information https://doc.rust-lang.org/book/ch03-02-data-types.html
