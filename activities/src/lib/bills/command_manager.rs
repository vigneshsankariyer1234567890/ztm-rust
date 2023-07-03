use super::command::command_type::*;
use super::bill_manager::BillManager;
use super::crud_command_manager::generate_crud_command;
use super::view_command_manager::generate_view_command;
use super::time_travel_command_manager::generate_time_travel_command;
use super::utils::split_input_by_char;
use super::command::exit_command::ExitCommand;
use super::command::help_command::HelpCommand;

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

  pub fn generate_command(&self, input: &str) -> Option<Box<dyn Command>> {
    let split_input = split_input_by_char(input, ' ');

    let command_word = split_input[0].as_str();

    let args = split_input[1..].to_vec();

    let command_type = CommandType::get_command_type(command_word)?;

    match command_type {
      CommandType::Add => generate_crud_command(&command_type, &args),
      CommandType::View => generate_view_command(&args),
      CommandType::Remove => generate_crud_command(&command_type, &args),
      CommandType::Edit => generate_crud_command(&command_type, &args),
      CommandType::Undo => generate_time_travel_command(&command_type, self.command_pointer, &self.command_stack),
      CommandType::Redo => generate_time_travel_command(&command_type, self.command_pointer, &self.command_stack),
      CommandType::Help => Some(Box::new(HelpCommand::of())),
      CommandType::Exit => Some(Box::new(ExitCommand::of())),
    }
  }
}