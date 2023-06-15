// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
  Backstage(String, f64),
  Vip(String, f64),
  Standard(f64)
}


fn main() {
  let tickets = vec![
    Ticket::Backstage("Vignesh".to_owned(), 100.0),
    Ticket::Vip("Shreya".to_owned(), 80.0),
    Ticket::Standard(30.0)
  ];

  for ticket in &tickets {
    match ticket {
      Ticket::Backstage(name, price) => println!("{:?} bought a backstage ticket with price ${:?}", name, price),
      Ticket::Vip(name, price) => println!("{:?} bought a VIP ticket with price ${:?}", name, price),
      Ticket::Standard(price) => match price {
        61.0.. => println!("This guy is a bit high class but can't really afford it, his price is {price:?}"),
        other => println!("This guys a loser, price is {other:?}")
      }
    }
  }
}
