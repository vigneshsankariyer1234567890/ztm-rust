use std::fmt::Debug;

use crate::bills::{bill_manager::BillManager, bill::Bill};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandType {
  Add,
  View,
  Remove,
  Edit,
  Undo,
  Redo,
  Help,
  Exit,
  Welcome,
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
      CommandType::Welcome => "welcome",
    }
}
}

pub trait Command: Debug {
  fn get_info(&self) -> String;

  fn get_command_args(&self) -> String;

  fn get_command_word(&self) -> String;

  fn get_command_type(&self) -> CommandType;

  fn as_command(&self) -> Box<dyn Command>;

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
  fn execute(&self, bill_manager: BillManager) -> ExecutionResult;

  fn clone_boxed_executable(&self) -> Box<dyn ExecutableCommand>;
}

impl Clone for Box<dyn ExecutableCommand> {
  fn clone(&self) -> Box<dyn ExecutableCommand> {
    self.clone_boxed_executable()
  }
}

pub trait CrudCommand: ExecutableCommand {
  fn get_inverse(&self) -> Box<dyn CrudCommand>;
  fn clone_boxed_crud(&self) -> Box<dyn CrudCommand>;
  fn with_possibly_previous_bill(&self, optional_bill: Option<Bill>) -> Box<dyn CrudCommand>;
}

impl Clone for Box<dyn CrudCommand> {
  fn clone(&self) -> Box<dyn CrudCommand> {
    self.clone_boxed_crud()
  }
}

pub trait TimeTravelCommand: Command {
  fn generate_new_crud_command(&self) -> Box<dyn CrudCommand>;

  fn get_new_position_of_command(&self) -> usize;
}

#[derive(Debug)]
pub struct ExecutionResult {
  pub bill_manager: Option<BillManager>,
  pub successful_executable_command: Box<dyn ExecutableCommand>,
}
