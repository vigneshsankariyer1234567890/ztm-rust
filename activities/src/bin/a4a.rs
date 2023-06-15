// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
  let am_i_sexy = true;

  match am_i_sexy {
    true => println!("You are the sexiest :3"),
    false => println!("No you're pretty ugly"),
  }
}
