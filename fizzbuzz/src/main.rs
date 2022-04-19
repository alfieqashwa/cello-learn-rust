fn main() {
    let result = fizzbuzz_vec(101);
    println!("{:#?}", result)
}

fn _fizzbuzz_println(upto: u64) {
    for i in 0..upto {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}

fn _fizzbuzz_ifelse(upto: u64) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in 0..upto {
        if i % 3 == 0 && i % 5 == 0 {
            result.push("FizzBuzz".into())
        } else if i % 3 == 0 {
            result.push("Fizz".into())
        } else if i % 5 == 0 {
            result.push("Buzz".into())
        } else {
            result.push(i.to_string())
        }
    }
    result
}

fn fizzbuzz_vec(upto: u64) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for i in 0..upto {
        result.push(match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".into(),
            (0, _) => "Fizz".into(),
            (_, 0) => "Buzz".into(),
            (_, _) => i.to_string(),
        })
    }
    return result;
}
