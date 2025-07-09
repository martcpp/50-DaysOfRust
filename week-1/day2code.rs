// variables


fn main() {

    // imutable variables
    let name = "John Doe";
    let age = 30;
    let is_student = true;
    let height = 1.75;

    // formatted string with placeholders
    println!("Hello, my name is {0}. I'm {1} years old. I'm a student: {2}. My height is {3} meters.", name, age, is_student, height);


    // mutable variables
    let mut weight = 75.0;
    let mut nationality = "American";
    weight += 10.0;
    nationality = "British";
    println!("My current weight is {} kg. I'm from {}.", weight, nationality);

    // variable shadowing
    let name = "Jane Doe";
    println!("My new name is {}", name);

    // variable scope
    {
        let name = "Alice";
        println!("Inside the scope: My name is {}", name);
    }


}


