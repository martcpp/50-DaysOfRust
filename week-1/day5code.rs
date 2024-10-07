// conditional and loop

fn main() {
    let mut number = 4;
    let ahlpa = 20;

    if ahlpa % 2 == 0 {
        println!("The number {} is even.", ahlpa);
    } else if ahlpa % 2 == 1 {
        println!("The number {} is odd.", ahlpa);
    }else{
        println!("The number {} is neither even nor odd.", ahlpa);
    }

    for i in 0..10 {
        println!("Loop iteration: {}", i);
    }

    while number % 2 == 0 {
        println!("The number {} is still even.", number);
        number += 1;
        if number > 10 {
            break;
        }
        continue;
    }

    loop {
        println!("Infinite loop iteration: {}", number);
        number += 1;
        if number > 10 {
            break;
        }
    }
}
