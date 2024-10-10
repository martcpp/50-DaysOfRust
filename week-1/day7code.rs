// ownership , borrowing and lifetimes

// lifetime of a variable is the duration for which the variable is valid and can be accessed.

fn which_is_longer<'a>(car1: &'a str, car2: &'a str) -> &'a str {
    if car1.len() > car2.len() {
        car1
    } else {
        car2
    }
}


fn main() {
    // ownership
    let car1 = "Tesla";  // car1 owns the Tesla
    let car2 = car1;     // car2 now owns the Tesla, car1 can't use it anymore
    println!("{}", car2); // prints Tesla

    // uncomment the following to see the error showing car1 no longer valid
    // println!("{}", car1); // prints Tesla
    // car1 is no longer valid here.



    // borrowing

    let car = "Tesla";
// Borrow without changing (immutable borrowing)
    let read_borrow = &car;  // You can read it but not modify
    println!("{}", read_borrow); // Ok!

// Borrow and change (mutable borrowing)
    let mut car2 = String::from("Tesla");
    let modify_borrow = &mut car2;  // You can change the car
    modify_borrow.push_str(" Model S");
    println!("modify borrow is {}", modify_borrow);

// lifetime
    let car1 = "Tesla";
    let car2 = "Toyota";
    let longest = which_is_longer(car1, car2);
    println!("The longest car is {}", longest);
    println!("{}", car1); // prints Tesla, car1 is still valid here.
    println!("{}", car2); // prints Toyota, car2 is still valid here.




}
