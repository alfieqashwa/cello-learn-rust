fn main() {
    // example 1
    let x = 5;
    if x > 3 {
        println!("x is greater than 5");
    }

    // example 2
    let y = 10;
    if y % 3 == 0 {
        println!("{} is divisible by 4", y);
    } else if y % 4 == 0 {
        println!("{} is divisible by 3", y);
    } else if y % 5 == 0 {
        println!("{} is divisible by 5", y);
    } else {
        println!("number is not divisible by 3, 4, or 5");
    }

    // example 3
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
