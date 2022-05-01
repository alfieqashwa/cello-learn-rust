fn main() {
    // loop
    println!("Loop example");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };
    println!("The result is {}", result);

    // while
    println!("While example");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    println!("For example");
    for element in (1..4).rev() {
        println!("{}!", element);
    }
    println!("LIFTOFF!!!");
}
