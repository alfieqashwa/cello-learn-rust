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

pub fn run() {
  integers_example();
  println!("===================");
  floating_example();
  println!("===================");
  boolean_example();
  println!("===================");
  character_example();
  println!("===================");
}

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
