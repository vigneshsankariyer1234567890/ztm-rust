// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned

fn longest<'a>(str_a: &'a str, str_b: &'a str) -> &'a str {
  if str_a.len() >= str_b.len() {
    str_a
  } else {
    str_b
  }
}

fn main() {
    let short = "this is along message";
    let long = "this is a long message";
    println!("{}", longest(short, long))
}
