use super::command_type::{Command, CommandType};
use crate::bills::bill_manager::BillManager;

pub struct ViewCommand {
  id: Option<String>,
  command_type: CommandType,
}

impl ViewCommand {
  pub fn of(id: Option<String>) -> Self {
    ViewCommand {
      id,
      command_type: CommandType::View,
    }
  }

  pub fn get_dummy_command() -> Self {
    Self::of(None)
  }
}

impl Command for ViewCommand {
  fn execute(&self, bill_manager: BillManager) -> Option<BillManager> {
    let name = self.id;

    println!("Looking for bills...");

    match name {
      Some(id) => bill_manager.view_bill(&id),
      None => bill_manager.view_bills(),
    }
  }

  fn get_info(&self) -> String {
    "View your bill(s)".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name (optional)>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::View.as_str().to_owned()
  }
}
