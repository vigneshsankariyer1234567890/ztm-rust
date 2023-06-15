// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
  Red,
  Blue,
  Green,
  Yellow,
}

fn match_color (color: Color) {
  match color {
    Color::Blue => println!("Blue"),
    Color::Green => println!("Green"),
    Color::Yellow => println!("Yellow"),
    Color::Red => println!("Red"),
  }
}

fn main() {
  let my_color = Color::Red;
  let my_friends_color = Color::Blue;
  let my_dads_color = Color::Green;
  let my_mums_color = Color::Yellow;

  match_color(my_color);
  match_color(my_dads_color);
  match_color(my_friends_color);
  match_color(my_mums_color);
}
