use super::command_type::{Command, CrudCommand, CommandType, TimeTravelCommand};

#[derive(Clone)]
pub struct UndoCommand {
  command_to_undo: Box<dyn CrudCommand>,
  command_position: i32,
  command_type: CommandType
}

impl UndoCommand {
  pub fn of(command_to_undo: Box<dyn CrudCommand>, command_position: i32) -> Self {
    UndoCommand {
      command_to_undo,
      command_position,
      command_type: CommandType::Undo
    }
  }
}

impl Command for UndoCommand {
  fn get_info(&self) -> String {
    "Undo your last command".to_owned()
  }

  fn get_command_args(&self) -> String {
    "".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Undo.as_str().to_owned()
  }

  fn get_command_type(&self) -> CommandType {
    self.command_type.clone()
  }

  fn as_command(&self) -> Box<dyn Command> {
    Box::new(self.clone())
  }

  fn as_time_travel_command(&self) -> Option<Box<dyn TimeTravelCommand>> {
    Some(Box::new(self.clone()))
  }
}

impl TimeTravelCommand for UndoCommand {
  fn generate_new_crud_command(&self) -> Box<dyn CrudCommand> {
    self.command_to_undo.get_inverse()
  }

  fn get_new_position_of_command(&self) -> i32 {
    self.command_position
  }
}
