mod no_parameters;
mod where_parameters_has_different_types_with_the_return_types;
mod with_parameters;
mod with_return_value;
mod with_return_value_and_parameters;
mod with_return_value_and_parameters_and_multiple_statements;

pub fn run() {
  no_parameters::run();
  println!("===============================");

  with_parameters::run(1, 2);
  println!("===============================");

  let result = with_return_value::run();
  assert_eq!(result, 42);
  println!("The return value is: {}", result);
  println!("===============================");

  let result = with_return_value_and_parameters::run(1, 2);
  assert_eq!(result, 3);
  println!("The return value is: {}", result);
  println!("===============================");

  let result = with_return_value_and_parameters_and_multiple_statements::run(1, 2);
  assert_eq!(result, 6);
  println!("The return value is: {}", result);
  println!("===============================");

  let tupl = (1, 2);
  let result = where_parameters_has_different_types_with_the_return_types::run(tupl);
  assert_ne!(result, true);
  assert_eq!(result, false);
  println!("The return value is: {}", result);
  println!("===============================");
}
