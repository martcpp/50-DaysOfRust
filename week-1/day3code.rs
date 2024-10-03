// datatype

fn main() {

    let age: u8 = 30;  // Integer (u8: unsigned 8-bit integer)

    let birth_year: i32 = 1993; // Integer (i32: signed 32-bit integer)

    let population: i64 = 7_800_000_000; // Integer (i64: signed 64-bit integer)

    let height: f32 = 1.8; // Floating point number (f32: 32-bit float)

    let weight: f64 = 75.5;   // Floating point number (f64: 64-bit float)

    let name: &str = "John Doe"; // String slice (&str: borrowed string, fixed length)

    let full_name: String = String::from("Jonathan Doe"); // String (String: growable heap-allocated string)

    let is_student: bool = true;  // Boolean (true or false)

    let grade: char = 'A';  // Character (char: single character, 4 bytes)

    let fav_numbers: [u8; 5] = [1, 2, 3, 4, 5]; // Array of unsigned 8-bit integers (fixed-size array)

    let person_info: (i32, f64, &str) = (30, 75.5, "John Doe"); // Tuple (fixed-size, can contain multiple types)

    let addresses: Vec<String> = vec![String::from("123 Main St"), String::from("456 Elm St")];  // Vector of Strings (dynamic, growable array of Strings)

    // Print the values
    println!("Age (u8): {}", age);
    println!("Birth year (i32): {}", birth_year);
    println!("World population (i64): {}", population);
    println!("Height (f32): {}", height);
    println!("Weight (f64): {}", weight);
    println!("Name (&str): {}", name);
    println!("Full name (String): {}", full_name);
    println!("Is student (bool): {}", is_student);
    println!("Grade (char): {}", grade);
    println!("Favorite numbers (array): {:?}", fav_numbers); // {:?} is used to print arrays
    println!("Person info (tuple): {:?}", person_info); // {:?} is used to print tuples
    println!("Addresses (Vec<String>): {:?}", addresses); // {:?} is used to print vectors

}
