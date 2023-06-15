// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io::stdin;

#[derive(Debug, Clone, Copy)]
enum PowerOption {
  Off,
  Sleep,
  Reboot,
  Shutdown,
  Hibernate,
  Invalid,
}

fn get_message(option: PowerOption) -> String {
  match option {
    PowerOption::Off => "switching off".to_owned(),
    PowerOption::Sleep => "sleeping now".to_owned(),
    PowerOption::Reboot => "rebooting computer".to_owned(),
    PowerOption::Shutdown => "shutting down".to_owned(),
    PowerOption::Hibernate => "hibernating".to_owned(),
    PowerOption::Invalid => "Error: Invalid Option".to_owned(),
  }
}

fn get_power_option(option_as_string: String) -> PowerOption {
  match option_as_string.as_str() {
    "off" => PowerOption::Off,
    "sleep" => PowerOption::Sleep,
    "reboot" => PowerOption::Reboot,
    "shutdown" => PowerOption::Shutdown,
    "hibernate" => PowerOption::Hibernate,
    _ => PowerOption::Invalid,
  }
}


fn main() {
  let mut input = String::new();

  println!("Enter your command:");

  stdin().read_line(&mut input)
    .expect("Failed to read line");

  println!("{}", input);

  let result = Option::Some(input)
    .map(|input| input.to_lowercase())
    .map(|lowercased| lowercased.trim().to_owned())
    .map(|sanitised_input| get_power_option(sanitised_input))
    .map(|power_option| get_message(power_option));

  match result {
    Some(res) => println!("Here is your command action: {:?}", res),
    None => println!("Something went wrong")
  }
}
