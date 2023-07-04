use super::command_type::{Command, CommandType, ExecutableCommand, ExecutionResult};
use crate::bills::bill_manager::BillManager;

#[derive(Debug, Clone)]
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
  fn get_info(&self) -> String {
    "View your bill(s)".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name (optional)>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::View.as_str().to_owned()
  }

  fn get_command_type(&self) -> CommandType {
    self.command_type.clone()
  }

  fn as_command(&self) -> Box<dyn Command> {
    Box::new(self.clone())
  }

  fn as_executable_command(&self) -> Option<Box<dyn ExecutableCommand>> {
    Some(Box::new(self.clone()))
  }
}

impl ExecutableCommand for ViewCommand {
  fn execute(&self, bill_manager: BillManager) -> ExecutionResult {
    let name = self.id.clone();

    println!("Looking for bills...");

    match name {
      Some(id) => ExecutionResult {
        bill_manager: bill_manager.view_bill(&id),
        successful_executable_command: self.clone_boxed_executable()
      },
      None => ExecutionResult {
        bill_manager: bill_manager.view_bills(),
        successful_executable_command: self.clone_boxed_executable()
      },
    }
  }

  fn clone_boxed_executable(&self) -> Box<dyn ExecutableCommand> {
    Box::new(self.clone())
  }
}
