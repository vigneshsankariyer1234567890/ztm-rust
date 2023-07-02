use super::command_type::{Command, CommandType, CrudCommand, ExecutableCommand};
use super::add_command::AddCommand;
use crate::bills::{bill_manager::BillManager, bill::Bill};
#[derive(Clone)]
pub struct RemoveCommand {
  id: String,
  command_type: CommandType,
  bill: Option<Bill>
}

impl RemoveCommand {
  pub fn of(id: String) -> Self {
    RemoveCommand {
      id,
      command_type: CommandType::Remove,
      bill: None,
    }
  }

  pub fn get_dummy_command() -> Self {
    Self::of(String::from(""))
  }

  pub fn with_bill(mut self, bill: Bill) -> Self {
    self.bill = Some(bill);
    self
  }
}

impl Command for RemoveCommand {
  fn get_info(&self) -> String {
    "Remove a bill".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name of bill>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Remove.as_str().to_owned()
  }

  fn get_command_type(&self) -> CommandType {
    self.command_type.clone()
  }

  fn as_crud_command(&self) -> Option<Box<dyn CrudCommand>> {
    Some(Box::new(self.clone()))
  }
}

impl ExecutableCommand for RemoveCommand {
  fn execute(&mut self, bill_manager: BillManager) -> Option<BillManager> {
    let bill_collection = bill_manager.get_bill_collection()?;
    let bill = bill_collection.get(self.id.as_str())?;
    self.clone().with_bill(bill.clone());
    println!("Removing bill with name {:?}", self.id);
    bill_manager.remove_bill(self.id.as_str())
  }
}

impl CrudCommand for RemoveCommand {
  fn get_inverse(&self) -> Box<dyn CrudCommand> {
    Box::new(
      AddCommand::of(self.bill.clone().unwrap())
    )
  }

  fn clone_box(&self) -> Box<dyn CrudCommand> {
    Box::new(self.clone())
  }
}