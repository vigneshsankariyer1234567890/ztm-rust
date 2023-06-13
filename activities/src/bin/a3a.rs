// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn are_you_sexy(answer: bool) {
  if answer {
    println!("You are sexy ;)");
  } else {
    println!("Nah you an ugly mofo :P");
  }
}

fn main() {
  let is_sexy: bool = true;
  are_you_sexy(is_sexy);
}
