use super::command::command_type::{Command, CommandType};
use super::command::add_command::AddCommand;
use super::command::edit_command::EditCommand;
use super::command::remove_command::RemoveCommand;
use super::bill::Bill;
use super::utils::split_input_by_char;

pub fn generate_crud_command(command_type: &CommandType, input_array: &Vec<String>) -> Option<Box<dyn Command>> {
  if input_array.len() > 2 {
    return None;
  }

  let name = input_array.get(0)?.as_str();

  let bill_amount_as_string = input_array.get(1)?.as_str();

  let split_input = split_input_by_char(bill_amount_as_string, '.');

  match command_type {
    CommandType::Remove => Some(
      Box::new(RemoveCommand::of(name.to_string()))
    ),
    CommandType::Add => {
      let bill = Bill::generate_bill(name, &split_input)?;
      Some(Box::new(AddCommand::of(bill)))
    },
    CommandType::Edit => {
      let bill = Bill::generate_bill(name, &split_input)?;
      Some(Box::new(EditCommand::of(bill)))
    },
    _ => None
  }
}