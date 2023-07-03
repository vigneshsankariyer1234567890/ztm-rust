use super::command_type::{Command, CrudCommand, CommandType, TimeTravelCommand};

#[derive(Clone)]
pub struct RedoCommand {
  command_to_redo: Box<dyn CrudCommand>,
  command_type: CommandType
}

impl RedoCommand {
  pub fn of(command_to_redo: Box<dyn CrudCommand>) -> Self {
    RedoCommand {
      command_to_redo,
      command_type: CommandType::Redo
    }
  }
}

impl Command for RedoCommand {
  fn get_info(&self) -> String {
    "Redo your last command".to_owned()
  }

  fn get_command_args(&self) -> String {
    "".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Redo.as_str().to_owned()
  }

  fn get_command_type(&self) -> CommandType {
    self.command_type
  }

  fn as_time_travel_command(&self) -> Option<Box<dyn TimeTravelCommand>> {
    Some(Box::new(self.clone()))
  }
}

impl TimeTravelCommand for RedoCommand {
  fn generate_new_crud_command(&self) -> Box<dyn CrudCommand> {
    self.command_to_redo.clone_box()
  }
}