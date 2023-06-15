// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
  name: String,
  age: i32,
}

// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
impl Adult {
  fn new(name: &str, age: i32) -> Result<Adult, String> {
    match age {
      21.. => Ok( Adult{ name: name.to_owned(), age }),
      _ => Err("Too young to be an adult".to_owned())
    }
  }
}

fn print_adult(adult: &Adult) {
  println!("Adult: {:?}", adult);
}

fn pick_adult(name: &str, age: i32) -> Result<(), String> {
  let adult = Adult::new(name, age)?;
  print_adult(&adult);
  Ok(())
}

fn main() {
  let actual_adult = pick_adult("Vignesh", 24);
  let child = pick_adult("Kavya", 18);

  println!("error: {:?}", child);
}
