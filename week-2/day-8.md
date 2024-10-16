# Structs

In Rust, a **struct**  is like a blueprint for creating custom types. Think of it like a form you fill out. Each struct can hold multiple pieces of related data, called **fields**, each with its own type.

for vidoe explanation of the struct visit here: https://www.youtube.com/watch?v=aSzyVVz57L8&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=21

Here's a simple way to understand it:

Imagine you're creating a blue print  to describe a book:

- **Title**: "Rust Programming"
- **Author**: "Mart RUST "
- **Pages**: 350

In Rust, you can create a **struct** called `Book` to hold this information. The **struct** would have fields for `title`, `author`, and `pages`, each with a specific data type. Then, when you want to describe a book, you'd fill in these fields, just like completing a form.

Here's an example in Rust:

```rust
struct Book {
    title: String,
    author: String,
    pages: u32, // u32 means a 32-bit unsigned integer for page numbers
}

fn main() {
    let my_book = Book {
        title: String::from("Rust Programming"),
        author: String::from("John Doe"),
        pages: 350,
    };

    println!("The book '{}' by {} has {} pages.", my_book.title, my_book.author, my_book.pages);
}
```

### Key Points:
- **Structs** group related information.
- Each field in the struct has a name (like "title" or "author") and a type (like `String` or `u32`).
- Once you define a struct, you can create instances of it (like `my_book` in the example) and fill in the data.
