use super::command_type::{Command, CommandType};

#[derive(Debug, Clone)]
pub struct ExitCommand {
  command_type: CommandType
}

impl ExitCommand {
  pub fn of() -> Self {
    ExitCommand {
      command_type: CommandType::Exit
    }
  }

  pub fn get_dummy_command() -> Self {
    Self::of()
  }
}

impl Command for ExitCommand {
  fn get_info(&self) -> String {
    String::from("Exit the application.")
  }

  fn get_command_args(&self) -> String {
    String::new() // No arguments needed for exit command.
  }

  fn get_command_word(&self) -> String {
    self.command_type.as_str().to_string()
  }

  fn get_command_type(&self) -> CommandType {
    self.command_type.clone()
  }

  fn as_command(&self) -> Box<dyn Command> {
    Box::new(self.clone())
  }
}
