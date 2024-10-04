// fuction

fn main() {
    println!("Hello, world!");
    greet("John Doe");
    println!("The sum of 5 and 3 is {}", add(5, 3));
    let (result, is_divisible) = divide(10.0, 2.0);
    println!("The result is {} and it is {} divisible by 2.", result, if is_divisible { "is" } else { "is not" });

}

// function with parameters

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// function with return value

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// function with multiple return values

fn divide(x: f64, y: f64) -> (f64, bool) {
    if y == 0.0 {
        (0.0, false)
    } else {
        (x / y, true)
    }
}


