use std::collections::HashMap;
use super::bill::Bill;

pub struct BillManager {
  bill_collection: HashMap<String, Bill>,
}

impl BillManager {
  pub fn new() -> Self {
    Self {
      bill_collection: HashMap::new(),
    }
  }

  pub fn of(bill_collection: HashMap<String, Bill>) -> Option<Self> {
    Some(
      BillManager{ bill_collection }
    )
  }

  pub fn get_bill_collection(&self) -> Option<HashMap<String, Bill>> {
    Some(self.bill_collection.clone())
  } 

  pub fn add_bill(&self, bill: &Bill) -> Option<Self> {
    let mut bill_collection = self.get_bill_collection()?;
    let name = bill.get_name();
    
    if bill_collection.contains_key(&name) {
      return None;
    }

    bill_collection.insert(name, bill.to_owned());
    BillManager::of(bill_collection)
  }

  pub fn remove_bill(&self, name: &str) -> Option<Self> {
    let mut bill_collection = self.get_bill_collection()?;

    if !bill_collection.contains_key(name) {
      return None;
    }

    let bill = bill_collection.remove(name)?;

    println!("Removed bill: {:?}", bill);

    BillManager::of(bill_collection)
  }

  pub fn edit_bill(&self, bill: &Bill) -> Option<Self> {
    let mut bill_collection = self.get_bill_collection()?;

    let name = bill.get_name();

    if !bill_collection.contains_key(&name) {
      return None;
    }

    let removed_bill = bill_collection.remove(&name)?;

    let added_bill = bill_collection.insert(name, bill.to_owned())?;

    println!("Edited bill {:?} to this: {:?}", removed_bill, added_bill);

    BillManager::of(bill_collection)
  }

  pub fn view_bills(&self) -> Option<Self> {
    self.bill_collection
      .iter()
      .enumerate()
      .for_each(|(index, bill)| println!("Bill {:?}: {:?}", index, bill));

    BillManager::of(self.get_bill_collection()?)
  }

  pub fn view_bill(&self, name: &str) -> Option<Self> {
    let found_bill = self.bill_collection.get(name);

    match found_bill {
      Some(bill) => println!("Found bill {:?}...", found_bill),
      None => println!("Didn't find any such bill."),
    };

    BillManager::of(self.get_bill_collection()?)
  }

  pub fn print_welcome_message(&self) -> Option<Self> {
    // Maybe some ascii art in the future?
    println!("Welcome to BillManager!");
    BillManager::of(self.get_bill_collection()?)
  }
}