// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn transform_word_to_uppercase(word: &str) -> String {
  return word.to_uppercase();
}

fn transform_word_to_lowercase(word: &str) -> String {
  return word.to_lowercase();
}

fn main() {
  let my_word = "hEllO";

  let uppercased = transform_word_to_uppercase(my_word);
  let lowercased = transform_word_to_lowercase(my_word);

  println!("The uppercase word: {uppercased:?}");
  println!("The lowercase word: {lowercased:?}");
}
