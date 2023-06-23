use crate::bills::bill_manager::BillManager;

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
  fn execute(&self, bill_manager: BillManager) -> Option<BillManager>;

  fn get_info(&self) -> String;

  fn get_command_args(&self) -> String;

  fn get_command_word(&self) -> String;
}
