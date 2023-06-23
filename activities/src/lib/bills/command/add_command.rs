use super::command_type::{Command, CommandType};
use crate::bills::bill::Bill;
use crate::bills::bill_manager::BillManager;

pub struct AddCommand {
  bill: Bill,
  command_type: CommandType
}

impl AddCommand {
  pub fn of(bill: Bill) -> Self {
    AddCommand {
      bill,
      command_type: CommandType::Add
    }
  }

  pub fn get_dummy_command() -> Self {
    Self::of(Bill {
      name: String::from(""),
      amount_in_dollars: 0,
      amount_in_cents: 0,
    })
  }
}

impl Command for AddCommand {
  fn execute(&self, bill_manager: BillManager) -> Option<BillManager> {
    let bill = self.bill;
    println!("Adding bill {:?}...", &bill);
    bill_manager.add_bill(&bill)
  }

  fn get_info(&self) -> String {
    "Add a bill to the manager".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name> <Value in dollars>.<Value in cents>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Add.as_str().to_owned()
  }
}
