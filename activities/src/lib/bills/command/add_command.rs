use super::command_type::{Command, CommandType, CrudCommand, ExecutableCommand, ExecutionResult};
use super::remove_command::RemoveCommand;
use crate::bills::bill::Bill;
use crate::bills::bill_manager::BillManager;

#[derive(Debug, Clone)]
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

  fn as_command(&self) -> Box<dyn Command> {
    Box::new(self.clone())
  }

  fn as_crud_command(&self) -> Option<Box<dyn CrudCommand>> {
    Some(Box::new(self.clone()))
  }

  fn as_executable_command(&self) -> Option<Box<dyn ExecutableCommand>> {
    Some(Box::new(self.clone()))
  }
}

impl ExecutableCommand for AddCommand {
  fn execute(&self, bill_manager: BillManager) -> ExecutionResult {
    let bill = self.bill.clone();
    println!("Adding bill {:?}...", &bill);
    ExecutionResult {
      bill_manager: bill_manager.add_bill(&bill),
      successful_executable_command: self.clone_boxed_executable()
    }
  }

  fn clone_boxed_executable(&self) -> Box<dyn ExecutableCommand> {
    Box::new(self.clone())
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

  fn clone_boxed_crud(&self) -> Box<dyn CrudCommand> {
    Box::new(self.clone())
  }

  fn with_possibly_previous_bill(&self, _optional_bill: Option<Bill>) -> Box<dyn CrudCommand> {
    Box::new(self.clone())
  }
}
