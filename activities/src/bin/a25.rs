// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter
trait Calculate {
  fn calculate_perimeter(&self) -> i32;
}

struct Square {
  side: i32,
}

impl Calculate for Square {
  fn calculate_perimeter(&self) -> i32 {
    self.side * 4
  }
}

struct Triangle {
  a: i32,
  b: i32,
  c: i32
}

impl Calculate for Triangle {
  fn calculate_perimeter(&self) -> i32 {
    self.a + self.b + self.c
  }
}

fn get_perimeter_of_calculate(calculable: impl Calculate) -> i32 {
  calculable.calculate_perimeter()
}

fn main() {
  let triangle = Triangle { a: 2, b: 3, c: 4 };

  let square = Square { side: 4 };

  println!("perimeter of triangle is {:?}", get_perimeter_of_calculate(triangle));
  println!("perimeter of square is {:?}", get_perimeter_of_calculate(square));
}
