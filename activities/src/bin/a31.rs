// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials
use std::fmt::Debug;

trait MaterialCost: Debug {
  fn calculate_material_cost(&self) -> i32;
}

#[derive(Debug)]
struct Wood {
  cost_per_sq_m: i32,
  number_of_sq_m: i32,
}

impl MaterialCost for Wood {
  fn calculate_material_cost(&self) -> i32 {
    self.cost_per_sq_m * self.number_of_sq_m
  }
}

#[derive(Debug)]
struct Tile {
  cost_per_sq_m: i32,
  number_of_sq_m: i32,
}

impl MaterialCost for Tile {
  fn calculate_material_cost(&self) -> i32 {
    self.cost_per_sq_m * self.number_of_sq_m
  }
}

#[derive(Debug)]
struct Carpet {
  cost_per_sq_m: i32,
  number_of_sq_m: i32,
}

impl MaterialCost for Carpet {
  fn calculate_material_cost(&self) -> i32 {
    self.cost_per_sq_m * self.number_of_sq_m
  }
}

fn main() {
  let material_vector: Vec<Box<dyn MaterialCost>> = vec![
    Box::new(Tile { cost_per_sq_m: 15, number_of_sq_m: 15 }),
    Box::new(Carpet { cost_per_sq_m: 10, number_of_sq_m: 10}),
    Box::new(Wood { cost_per_sq_m: 20, number_of_sq_m: 20 })
  ];

  material_vector.iter().for_each(|m| println!("Total cost of {:?} is {:?}\n", m, m.calculate_material_cost()));
  
}
