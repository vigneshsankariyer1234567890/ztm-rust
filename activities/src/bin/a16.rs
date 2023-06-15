// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Locker {
  name: String,
  assignment: Option<i32>,
}

fn main() {
  let marks_locker = Locker {
    name: "Mark".to_owned(),
    assignment: None,
  };

  println!("Name: {:?}", marks_locker.name);

  match marks_locker.assignment {
    Some(number) => println!("Mark's assignment is {:?}", number),
    None => println!("Mark didn't assign any number"),
  }
}
