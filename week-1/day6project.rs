use std::io;


fn main(){

    // read input from command line
    println!("Enter a number 1:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter a number 2:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    println!("what operation do you want to do (+,-,/,*)");
    let mut opps= String::new();
    std::io::stdin().read_line(&mut opps).expect("Failed to read line");

    // eliminate whites spaces from the input
    let num1: f32 = input1.trim().parse().expect("Please input a number");
    let num2: f32 = input2.trim().parse().expect("Please input a number");
    let opp = opps.trim();

    // perform the operation
    if opp == "+" {
        let result = num1 + num2; // addition
        println!("The sum is: {}", result);
    } else if opp == "-" {
        let result = num1 - num2; // substraction
        println!("The difference is: {}", result);
    } else if opp == "/" {
        if num2 == 0.0 {
            println!("Error: Division by zero is not allowed");
        } else {
            let result = num1 / num2; // division
            println!("The division is: {}",  result);
        }

    } else if opp == "*" {
       let result = num1 * num2; // multiplication
        println!("The product is: {}",  result );

    } else {
        println!("Invalid operation. Please use +,-,/,*");
    }


}
