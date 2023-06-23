use super::command::command_type::*;
use super::bill_manager::BillManager;

pub struct CommandManager {
  bill_manager: BillManager,
  command_stack: Vec<Box<dyn Command>>,
  command_pointer: i32,
}

impl CommandManager {
  pub fn of(bill_manager: BillManager, command_stack: Vec<Box<dyn Command>>) -> Self {
    Self {
      bill_manager,
      command_stack,
      command_pointer: -1,
    }
  }

  fn split_input_by_char(input: &str, character: char) -> Vec<String> {
    input.split(character)
      .map(|s| s.to_string())
      .collect()
  }

  pub fn generate_command(input: &str) -> Option<Box<dyn Command>> {
    let split_input = CommandManager::split_input_by_char(input, ' ');

    let command_word = split_input[0];

    let command_type = CommandType::get_command_type(&command_word)?;

    
  }
}