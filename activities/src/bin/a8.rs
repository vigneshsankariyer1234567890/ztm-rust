// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
  Apple,
  Grape,
  Passionfruit,
  Cola
}

struct Drink {
  flavour: Flavor,
  ounces: i32,
}

fn get_flavor(flavor: Flavor) -> &'static str {
  return match flavor {
    Flavor::Apple => "Apple",
    Flavor::Cola => "Cola",
    Flavor::Grape => "Grape",
    Flavor::Passionfruit => "Passionfruit",
  };
}

fn get_drink_info(drink: Drink) {
  let drink_flavor = get_flavor(drink.flavour);
  let drink_ounces = drink.ounces;

  println!("The flavor is {:?}.", drink_flavor);
  println!("The number of ounces is {:?}.", drink_ounces);
}

fn main() {
  let grape_soda = Drink {
    flavour: Flavor::Grape,
    ounces: 2,
  };

  get_drink_info(grape_soda);
}
