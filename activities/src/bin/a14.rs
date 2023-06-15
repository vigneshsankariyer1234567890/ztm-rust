// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

enum Color {
  Red,
  Blue,
  Green,
}

impl Color {
  fn print(&self) {
    match self {
      Color::Blue => println!("Blue"),
      Color::Red => println!("Red"),
      Color::Green => println!("Green"),
    };
  }
}

struct Person {
  age: i32,
  name: String,
  color: Color,
}

impl Person {
  fn print(&self) {
    println!("Person's name is {:?}", self.name);
    println!("Person's age is {:?}", self.age);
    self.color.print();
  }
}

fn main() {
  let people = vec![
    Person {
      age: 24,
      name: "Vignesh".to_owned(),
      color: Color::Blue,
    },
    Person {
      age: 23,
      name: String::from("Shreya"),
      color: Color::Red,
    },
    Person {
      age: 56,
      name: "Sankar".to_owned(),
      color: Color::Green,
    }
  ];

  for person in &people {
    if person.age > 23 {
      person.print();
    }
  }

}
