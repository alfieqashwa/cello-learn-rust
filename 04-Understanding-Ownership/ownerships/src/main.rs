/*
Ownership Rules:
   - Each value in Rust has a variable that’s called its owner.
   - There can only be one owner at a time.
   - When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    // string literal
    let str1 = "cello string literal";
    let str2 = str1; // the data is in the stack memory, not in the heap. So it's inexpensive-copy

    println!("str1: {}, str2: {}", str1, str2);

    // string type
    let string1 = String::from("Cello String");
    let string2 = string1.clone(); // deep copy -> expensive-copy
    println!("string1: {}, string2: {}", string1, string2);

    let string3 = String::from("Cello World!");
    let string4 = &string3; // reference -> inexpensive-copy

    println!("string3: {}, string4: {}", string3, string4);

    let mut string5 = String::from("Qashwa");
    string5.push_str(" and Cello");
    assert_eq!(string5, "Qashwa and Cello");
    println!("string5: {}", string5);
}
