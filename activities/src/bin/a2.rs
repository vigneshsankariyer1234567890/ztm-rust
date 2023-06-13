// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

fn display_number(number: i32) {
  println!("{:?}", number);
}

fn main() {
  let a: i32 = 10;
  let b: i32 = 20;
  let result: i32 = add(a, b);
  display_number(result);
}
