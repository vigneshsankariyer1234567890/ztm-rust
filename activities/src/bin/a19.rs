// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

#[derive(Debug)]
enum FurnitureType {
  Chair,
  Bed,
  Table,
  Couch,
  Invalid
}

#[derive(Debug)]
struct FurnitureOrder {
  furniture_type: FurnitureType,
  number: u8,
}

impl FurnitureOrder {
  fn get_furniture(furniture_name: &str, number_of_furniture: u8) -> Self {
    match furniture_name {
      "chair" => FurnitureOrder { furniture_type: FurnitureType::Chair, number: number_of_furniture },
      "bed" => FurnitureOrder { furniture_type: FurnitureType::Bed, number: number_of_furniture },
      "table" => FurnitureOrder { furniture_type: FurnitureType::Table, number: number_of_furniture },
      "couch" => FurnitureOrder { furniture_type: FurnitureType::Couch, number: number_of_furniture },
      _ => FurnitureOrder { furniture_type: FurnitureType::Invalid, number: 0 },
    }
  }
}

fn main() {
  let mut db: HashMap<String, FurnitureOrder> = HashMap::new();

  let chair_name = "chair";
  let bed_name = "bed";
  let table_name = "table";
  let couch_name = "couch";

  let chairs = FurnitureOrder::get_furniture(&chair_name, 5);
  db.insert(chair_name.to_owned(), chairs);

  let beds = FurnitureOrder::get_furniture(&bed_name, 3);
  db.insert(bed_name.to_owned(), beds);

  let tables = FurnitureOrder::get_furniture(&table_name, 2);
  db.insert(table_name.to_owned(), tables);

  let couches = FurnitureOrder::get_furniture(&couch_name, 0);
  db.insert(couch_name.to_owned(), couches);

  let mut total_stock = 0;

  for (_, furniture_order) in db.iter() {
    let number = furniture_order.number;

    total_stock = total_stock + number;

    let furniture_type = &furniture_order.furniture_type;
    match number {
      0 => println!("Furniture: {:?} is out of stock.", furniture_type),
      _ => println!("Furniture: {:?} has {:?} units", furniture_type, number)
    }
  }

  println!("Total stock: {:?}", total_stock);
}
