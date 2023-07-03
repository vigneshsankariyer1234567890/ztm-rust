use crate::bills::bill_manager::BillManager;

#[derive(Clone, Copy)]
pub enum CommandType {
  Add,
  View,
  Remove,
  Edit,
  Undo,
  Redo,
  Help,
  Exit,
}

impl CommandType {
  pub fn get_command_type(input: &str) -> Option<Self> {
    match input {
      "add" => Some(CommandType::Add),
      "view" => Some(CommandType::View),
      "remove" => Some(CommandType::Remove),
      "edit" => Some(CommandType::Edit),
      "undo" => Some(CommandType::Undo),
      "redo" => Some(CommandType::Redo),
      "help" => Some(CommandType::Help),
      "exit" => Some(CommandType::Exit),
      _ => None,
    }
  }

  pub fn as_str(&self) -> &'static str {
    match *self {
      CommandType::Add => "add",
      CommandType::Remove => "remove",
      CommandType::Edit => "edit",
      CommandType::View => "view",
      CommandType::Undo => "undo",
      CommandType::Redo => "redo",
      CommandType::Help => "help",
      CommandType::Exit => "exit",
    }
}
}

pub trait Command {
  fn get_info(&self) -> String;

  fn get_command_args(&self) -> String;

  fn get_command_word(&self) -> String;

  fn get_command_type(&self) -> CommandType;

  fn as_crud_command(&self) -> Option<Box<dyn CrudCommand>> {
    None
  }

  fn as_executable_command(&self) -> Option<Box<dyn ExecutableCommand>> {
    None
  }

  fn as_time_travel_command(&self) -> Option<Box<dyn TimeTravelCommand>> {
    None
  }
}

pub trait ExecutableCommand: Command {
  fn execute(&mut self, bill_manager: BillManager) -> Option<BillManager>;
}

pub trait CrudCommand: ExecutableCommand {
  fn get_inverse(&self) -> Box<dyn CrudCommand>;
  fn clone_box(&self) -> Box<dyn CrudCommand>;
}

impl Clone for Box<dyn CrudCommand> {
  fn clone(&self) -> Box<dyn CrudCommand> {
    self.clone_box()
  }
}

pub trait TimeTravelCommand: Command {
  fn generate_new_crud_command(&self) -> Box<dyn CrudCommand>;
}
