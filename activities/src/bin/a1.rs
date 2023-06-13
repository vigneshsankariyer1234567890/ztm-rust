// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn display_name(name: &str) {
  println!("{}", name);
}

fn main() {
  let first_name: &str = "Vignesh";
  let last_name: &str = "Iyer";
  display_name(first_name);
  display_name(last_name);
}
