// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
  let numbers = vec![10, 20, 30, 40];

  // need to apply borrow checker here since value is being borrowed in next scope
  // if i used if block instead, would have to apply borrow on number since 
  // if block would own number, making it unusable
  for number in &numbers {
    match number {
      30 => println!("thirty"),
      _ => println!("{:?}", number),
    }
  }

  let len = numbers.len();

  println!("The size is {len:?}");
}
