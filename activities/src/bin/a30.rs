// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

#[derive(Debug)]
enum BodyType {
  Truck,
  Car,
  Scooter,
}

#[derive(Debug)]
enum ColorType {
  Red,
  White,
  Black
}

trait Body {
  fn get_body_type(&self) -> BodyType;
}

struct Scooter {}
impl Body for Scooter {
  fn get_body_type(&self) -> BodyType {
    BodyType::Scooter
  }
}

struct Truck {}
impl Body for Truck {
  fn get_body_type(&self) -> BodyType {
      BodyType::Truck
  }
}

trait Color {
  fn get_color_type(&self) -> ColorType;
}
struct Red {}
impl Color for Red {
  fn get_color_type(&self) -> ColorType {
    ColorType::Red
  }
}
struct Black{}
impl Color for Black {
  fn get_color_type(&self) -> ColorType {
    ColorType::Black
  }
}

struct Vehicle<T, U> where 
  T: Body,
  U: Color
{
  body: T,
  color: U
}

impl <T, U> Vehicle<T, U> where
  T: Body,
  U: Color
{
  fn new(body: T, color: U) -> Self {
    Self {
      body,
      color
    }
  }

  fn get_details(&self) {
    println!("I am a {:?}, in {:?} color", self.body.get_body_type(), self.color.get_color_type())
  }
}

fn main() {
  let red_scooter = Vehicle::new(Scooter{}, Red{});
  let black_truck = Vehicle::new(Truck{}, Black{});

  red_scooter.get_details();
  black_truck.get_details();
}
