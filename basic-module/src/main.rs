mod basic;
use basic::{add_num, divide_num, multiply_num, subtract_num};

fn main() {
    let (x, y) = (6, 3);

    let (add_result, subtract_result, multiply_result, divide_result) = (
        add_num::add(x, y),
        subtract_num::subtract(x, y),
        multiply_num::multiply(x, y),
        divide_num::divide(x, y),
    );

    assert_eq!(x, 6);
    assert_eq!(y, 3);
    assert_eq!(add_result, 9);
    assert_eq!(subtract_result, 3);
    assert_eq!(multiply_result, 18);
    assert_eq!(divide_result, 2);
}
