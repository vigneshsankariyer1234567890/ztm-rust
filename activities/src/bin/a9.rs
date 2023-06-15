// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn get_coordinates() -> (i32, i32) {
  return (5, 5);
}

fn get_status_of_number(number: i32) -> &'static str {
  return match number {
    5 => "equals 5",
    6.. => "greater than 5",
    _ => "less than 5"
  }
}

fn main() {
  let (_x, y) = get_coordinates();

  let y_status = get_status_of_number(y);

  println!("The y-value, {y:?}, is {y_status:?}.");
}
