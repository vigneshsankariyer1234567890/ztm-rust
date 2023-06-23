use super::command_type::{Command, CommandType};
use crate::bills::bill::Bill;
use crate::bills::bill_manager::BillManager;

pub struct EditCommand {
  bill: Bill,
  command_type: CommandType
}

impl EditCommand {
  pub fn of(bill: Bill) -> Self {
    EditCommand {
      bill,
      command_type: CommandType::Edit,
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

impl Command for EditCommand {
  fn execute(&self, bill_manager: BillManager) -> Option<BillManager> {
    let bill = self.bill;
    println!("Editing bill {:?} if exists...", &bill);
    bill_manager.edit_bill(&bill)
  }

  fn get_info(&self) -> String {
    "Edit a bill in the manager".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name> <New value in dollars>.<New value in cents>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Edit.as_str().to_owned()
  }
}
