// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn get_word_format_of_number(number: i32) -> &'static str {
  match number {
    1 => return "one",
    2 => return "two",
    3 => return "three",
    _ => return "other",
  }
}

fn main() {
  let one = get_word_format_of_number(1);
  let two = get_word_format_of_number(2);
  let three = get_word_format_of_number(3);
  let zero = get_word_format_of_number(0);

  println!("{:?}", one);
  println!("{:?}", two);
  println!("{:?}", three);
  println!("{:?}", zero);
}
