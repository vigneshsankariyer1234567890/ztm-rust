// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats
use chrono::prelude::*;

fn main() {
  let utc = Utc::now();

  println!("Time now is {:?}", utc);

  let date = utc.format("%a %b %e %T %Y").to_string();

  println!("Time now is {:?}", date);
}
