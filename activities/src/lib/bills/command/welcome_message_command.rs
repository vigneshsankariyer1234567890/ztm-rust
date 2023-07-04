use super::command_type::{Command, CommandType, ExecutableCommand, ExecutionResult};
use crate::bills::bill_manager::BillManager;

#[derive(Debug, Clone)]
pub struct WelcomeMessageCommand {
  command_type: CommandType,
}

impl WelcomeMessageCommand {
  pub fn of() -> Self {
    WelcomeMessageCommand {
      command_type: CommandType::Welcome
    }
  }
}

impl Command for WelcomeMessageCommand {
  fn get_info(&self) -> String {
    "Get your welcome message".to_owned()
  }

  fn get_command_args(&self) -> String {
    "".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Welcome.as_str().to_owned()
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

impl ExecutableCommand for WelcomeMessageCommand {
  fn execute(&self, bill_manager: BillManager) -> ExecutionResult {
    ExecutionResult {
      bill_manager: bill_manager.print_welcome_message(),
      successful_executable_command: self.clone_boxed_executable()
    }
  }

  fn clone_boxed_executable(&self) -> Box<dyn ExecutableCommand> {
    Box::new(self.clone())
  }
}