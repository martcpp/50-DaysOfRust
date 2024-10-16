// struct

    // Person struct definition with fields and methods
    // `#[derive(Debug)]` attribute for printing the struct in a human-readable format
struct Person {
    name: String,
    age: u8,
    nationality: String,
}

fn main() {
    let person1 = Person {
        name: String::from("John"),
        age: 30,
        nationality: String::from("American"),
    };
    //println!("{:?}", person1);
    println!("Name: {}, Age: {}, Nationality: {}", person1.name, person1.age, person1.nationality);

    let person2 = Person {
        name: String::from("Jane"),
        age: 25,
        nationality: String::from("British"),
    };
    //println!("{:?}", person2);
    println!("Name: {}, Age: {}, Nationality: {}", person2.name, person2.age, person2.nationality);

    let mut person3 = Person {
        name: String::from("Alice"),
        age: 28,
        nationality: String::from("American"),
    };

    person3.age += 1;
    //println!("{:?}", person3);
    println!("Name: {}, Age: {}, Nationality: {}", person3.name, person3.age, person3.nationality);
}
