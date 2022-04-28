// SOURCE: https://doc.rust-lang.org/book/ch03-02-data-types.html

// Two data types: Scalar and compound.

/*
A scalar type is represented a single value.
Rust has 4 (four) primary of Scalar types:
    - integers:
        - signed: i8,i16, ..., i128, and isize.
        - unsigned: u8, u16, u32, ..., u128, and usize.

    - floating-point numbers:
        - f32, f64.

    - boolean:
        - bool.

    - characters:
        - char.
        (single-byte Unicode characters).
*/

/*
Compound types can group multiple values into one type.
Rust has two primitive compound types: tuples and arrays.
*/

fn integers_example() {
    let unsigned_num: u16 = 1;
    let unsigned_num = unsigned_num + 1_000;
    assert_eq!(unsigned_num, 1001);
    println!("unsigned_num is {}", unsigned_num);

    let signed_num: i16 = -1;
    let signed_num = signed_num - 1_000;
    assert_eq!(signed_num, -1001);
    println!("signed_num is {}", signed_num);
}

fn floating_example() {
    let float_num: f32 = 1.5;
    let float_num = float_num + 1.5;
    assert_eq!(float_num, 3.0);
    println!("float_num is {}", float_num);
}

fn boolean_example() {
    let is_true: bool = true;
    let is_true = !is_true;
    assert_eq!(is_true, false);
    println!("Is the value is true? {}", is_true);
}

fn character_example() {
    let character_example: char = 'c';
    assert_eq!(character_example, 'c');
    println!("The character_example is {}", character_example);
    let character_example = '😻';
    assert_eq!(character_example, '😻');
    println!("character_example until shadowing is {}", character_example);
}

fn scalar_types() {
    integers_example();
    println!("===================");
    floating_example();
    println!("===================");
    boolean_example();
    println!("===================");
    character_example();
    println!("===================");
}

/*
=== The Tuple Type ===
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

=== The Array Type ===
Unlike a tuple, every element of an array must have the same type.
Unlike arrays in some other languages, arrays in Rust have a fixed length.

Arrays are useful when you want your data allocated on the stack rather than the heap
or when you want to ensure you always have a fixed number of elements.
*/

fn tuple_example() {
    let tuple: (char, u8, bool, f32) = ('a', 1, true, 19.31);
    println!("tuple is {:?}", tuple);

    // destructuring
    let cars = ("BMW", "Audi", "Mercedes");
    let (car1, car2, car3) = cars;
    println!("car1 is {}", car1);
    println!("car2 is {}", car2);
    println!("car3 is {}", car3);

    let s = ("cello world", String::from("hello world"));
    println!(
        "string literal example: {},\nstring object example: {}",
        s.0, s.1
    );
}

fn array_example() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months: {:?}", months);
    println!("===================");
    println!("months in prettier output: {:#?}", months);
    println!("===================");
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("nums: {:?}", nums);
    let a = [10; 5];
    println!("{:#?}", a);

    let vals = [6, 7, 8, 9, 10];
    let eight = vals[2];
    let six_to_nine = &vals[0..4];
    let six_til_nine = &vals[0..=4];
    println!(
        "eight: {},\nsix_to_nine: {:?}\nsix_til_nine: {:?}",
        eight, six_to_nine, six_til_nine
    );

    let numbers = [1, 2, 3];
    for i in numbers.iter().rev() {
        println!("COUNTDOWN {:?}", i);
        println!("HAPPY HOLIDAYS!");
    }
}

fn compound_types() {
    tuple_example();
    println!("===================");
    array_example();
}

fn main() {
    scalar_types();
    compound_types();
}
