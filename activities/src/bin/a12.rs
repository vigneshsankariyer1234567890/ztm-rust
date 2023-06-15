// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
  dimensions: (i32, i32),
  weight: f64,
  color: &'static str,
}

impl Box {
  fn create_box() -> Self {
    Self { dimensions: (10, 10), weight: 100.0, color: "Red",}
  }

  fn display_characteristics(&self) {
    let (x, y) = self.dimensions;
    let weight = self.weight;
    let color = self.color;

    println!("The dimensions are {x:?} by {y:?}.");
    println!("The weight is {weight:?}");
    println!("The color is {color:?}");
  }
}

fn main() {
  let created_box = Box::create_box();
  created_box.display_characteristics();
}