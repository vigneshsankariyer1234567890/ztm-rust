// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pressure(u16);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
  let tile_vec = vec![
    Tile::Dirt,
    Tile::Water(Pressure(9)),
    Tile::Water(Pressure(10)),
    Tile::Brick(BrickStyle::Dungeon),
    Tile::Brick(BrickStyle::Gray),
    Tile::Treasure(TreasureChest { content: TreasureItem::Gold, amount: 100 }),
    Tile::Wood,
  ];
  
  for tile in tile_vec {
    print_tile_details(tile);
  }
}

fn print_tile_details(tile: Tile) {
  match tile {
    Tile::Brick(s) => {
      if s == BrickStyle::Dungeon {
        println!("{:?} brick", s);
      } else {
        println!("The brick color is {:?}", s);
      }
    },
    Tile::Water(p) => match p.0 {
      10.. => println!("High water pressure"),
      pressure => println!("Water pressure level: {:?}", pressure),
    },
    Tile::Dirt |
    Tile::Grass |
    Tile::Sand => println!("Ground tile"),
    Tile::Treasure(treasure_chest) => match treasure_chest {
      TreasureChest { 
        amount: 100.., content: TreasureItem::Gold
      } => println!("Lots of gold"),
      _ => ()
    },
    _ => ()
  }
}
