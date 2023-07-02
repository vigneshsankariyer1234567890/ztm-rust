use super::command_type::{Command, CommandType, CrudCommand, ExecutableCommand};
use super::remove_command::RemoveCommand;
use crate::bills::bill::Bill;
use crate::bills::bill_manager::BillManager;

#[derive(Clone)]
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
  fn get_info(&self) -> String {
    "Add a bill to the manager".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name> <Value in dollars>.<Value in cents>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Add.as_str().to_owned()
  }

  fn get_command_type(&self) -> CommandType {
    self.command_type
  }

  fn as_crud_command(&self) -> Option<Box<dyn CrudCommand>> {
    Some(Box::new(self.clone()))
  }
}

impl ExecutableCommand for AddCommand {
  fn execute(&mut self, bill_manager: BillManager) -> Option<BillManager> {
    let bill = self.bill.clone();
    println!("Adding bill {:?}...", &bill);
    bill_manager.add_bill(&bill)
  }
}

impl CrudCommand for AddCommand {
  fn get_inverse(&self) -> Box<dyn CrudCommand> {
    let bill = self.bill.clone();
    let bill_name = bill.get_name();
    Box::new(
      RemoveCommand::of(bill_name).with_bill(bill.clone())
    )
  }

  fn clone_box(&self) -> Box<dyn CrudCommand> {
    Box::new(self.clone())
  }
}
