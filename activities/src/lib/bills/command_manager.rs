use std::convert::TryInto;
use super::bill::Bill;
use super::command::add_command::AddCommand;
use super::command::command_type::*;
use super::bill_manager::BillManager;
use super::command::edit_command::EditCommand;
use super::command::exit_command::ExitCommand;
use super::command::help_command::HelpCommand;
use super::command::redo_command::RedoCommand;
use super::command::remove_command::RemoveCommand;
use super::command::undo_command::UndoCommand;
use super::command::view_command::ViewCommand;

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

  fn generate_amount_for_bill(input_array: &Vec<String>) -> Option<(i32, i32)> {
    if input_array.len() != 2 {
      return None;
    }

    let amount_in_dollars = input_array[0].parse::<i32>().ok()?;
    let amount_in_cents = input_array[1].parse::<i32>().ok()?;

    Some((amount_in_dollars, amount_in_cents))
  }

  fn generate_bill(name: &str, input_array: &Vec<String>) -> Option<Bill> {
    let (
      amount_in_dollars,
      amount_in_cents
    ) = CommandManager::generate_amount_for_bill(input_array)?;

    Bill::new(name, amount_in_dollars, amount_in_cents)
  }

  fn generate_crud_command(command_type: &CommandType, input_array: &Vec<String>) -> Option<Box<dyn Command>> {
    if input_array.len() > 2 {
      return None;
    }

    let name = input_array.get(0)?.as_str();

    let bill_amount_as_string = input_array.get(1)?.as_str();

    let split_input = CommandManager::split_input_by_char(bill_amount_as_string, '.');

    match command_type {
      CommandType::Remove => Some(
        Box::new(RemoveCommand::of(name.to_string()))
      ),
      CommandType::Add => {
        let bill = CommandManager::generate_bill(name, &split_input)?;
        Some(Box::new(AddCommand::of(bill)))
      },
      CommandType::Edit => {
        let bill = CommandManager::generate_bill(name, &split_input)?;
        Some(Box::new(EditCommand::of(bill)))
      },
      _ => None
    }
  }

  fn generate_view_command(input_array: &Vec<String>) -> Option<Box<dyn Command>> {
    let optional_id = input_array.get(0).map(|id| id.to_owned());

    Some(
      Box::new(ViewCommand::of(optional_id))
    )
  }

  fn generate_undo_command(&self) -> Option<Box<dyn Command>> {
    if self.command_pointer < 0 {
      return None;
    }

    // look for first command from command_pointer reversed which is a crud command
    let command = self.command_stack.iter()
      .enumerate()
      .rev()
      .filter(|(index, _value)| *index <= self.command_pointer.try_into().unwrap())
      .filter_map(|(_i, command)| command.as_crud_command())
      .next()?;

    Some(Box::new(UndoCommand::of(command)))
  }

  fn generate_redo_command(&self) -> Option<Box<dyn Command>> {
    let last_elem_index = self.command_stack.len() - 1;
    if self.command_pointer >= last_elem_index.try_into().unwrap() {
      // pointing to latest element, no more elements
      return None;
    }

    // look for first command from command_pointer to command_stack.len() which is a crud command
    let command = self.command_stack.iter()
      .enumerate()
      .filter(|(index, _value)| *index > self.command_pointer.try_into().unwrap())
      .find_map(|(_i, command)| command.as_crud_command())?;

    Some(Box::new(RedoCommand::of(command)))
  }

  fn generate_time_travel_command(&self, command_type: &CommandType) -> Option<Box<dyn Command>> {
    match command_type {
      CommandType::Undo => self.generate_undo_command(),
      CommandType::Redo => self.generate_redo_command(),
      _ => None,
    }
  }

  pub fn generate_command(&self, input: &str) -> Option<Box<dyn Command>> {
    let split_input = CommandManager::split_input_by_char(input, ' ');

    let command_word = split_input[0].as_str();

    let args = split_input[1..].to_vec();

    let command_type = CommandType::get_command_type(command_word)?;

    match command_type {
      CommandType::Add => CommandManager::generate_crud_command(&command_type, &args),
      CommandType::View => CommandManager::generate_view_command(&args),
      CommandType::Remove => CommandManager::generate_crud_command(&command_type, &args),
      CommandType::Edit => CommandManager::generate_crud_command(&command_type, &args),
      CommandType::Undo => self.generate_time_travel_command(&command_type),
      CommandType::Redo => self.generate_time_travel_command(&command_type),
      CommandType::Help => Some(Box::new(HelpCommand::of())),
      CommandType::Exit => Some(Box::new(ExitCommand::of())),
    }
  }
}