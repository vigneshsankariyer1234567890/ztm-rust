use super::command_type::{Command, CommandType};
use crate::bills::bill_manager::BillManager;

pub struct RemoveCommand {
  id: String,
  command_type: CommandType,
}

impl RemoveCommand {
  pub fn of(id: String) -> Self {
    RemoveCommand {
      id,
      command_type: CommandType::Remove,
    }
  }

  pub fn get_dummy_command() -> Self {
    Self::of(String::from(""))
  }
}

impl Command for RemoveCommand {
  fn execute(&self, bill_manager: BillManager) -> Option<BillManager> {
    println!("Removing bill with name {:?}", self.id);
    bill_manager.remove_bill(&(self.id))
  }

  fn get_info(&self) -> String {
    "Remove a bill".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name of bill>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Remove.as_str().to_owned()
  }
}